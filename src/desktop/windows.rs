//! Dedicated Win32 UI thread. Passive effects never receive pointer input.
use super::{drawing, Controller};
use crate::{
    displays::Display,
    presentation::{zoom_origin, Mode, Point, Scene},
    windows_displays,
};
use gtk4::cairo::{Context, Format, ImageSurface, Operator};
use std::{
    ptr::{null, null_mut},
    sync::Arc,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::{LibraryLoader::GetModuleHandleW, RemoteDesktop::*},
    UI::{HiDpi::*, Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

struct Buffer {
    dc: HDC,
    bitmap: HBITMAP,
    old: HGDIOBJ,
    bits: *mut u8,
    surface: ImageSurface,
    width: i32,
    height: i32,
}
impl Buffer {
    unsafe fn new(width: i32, height: i32) -> anyhow::Result<Self> {
        anyhow::ensure!(
            width > 0 && height > 0 && i64::from(width) * i64::from(height) <= 33_177_600,
            "Unsupported monitor size"
        );
        let dc = CreateCompatibleDC(null_mut());
        anyhow::ensure!(!dc.is_null(), "CreateCompatibleDC failed");
        let mut info: BITMAPINFO = std::mem::zeroed();
        info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = width;
        info.bmiHeader.biHeight = -height;
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;
        let mut bits = null_mut();
        let bitmap = CreateDIBSection(dc, &info, DIB_RGB_COLORS, &mut bits, null_mut(), 0);
        if bitmap.is_null() {
            DeleteDC(dc);
            anyhow::bail!("CreateDIBSection failed");
        }
        let old = SelectObject(dc, bitmap);
        let surface = match ImageSurface::create(Format::ARgb32, width, height) {
            Ok(s) => s,
            Err(e) => {
                SelectObject(dc, old);
                DeleteObject(bitmap);
                DeleteDC(dc);
                return Err(e.into());
            }
        };
        Ok(Self {
            dc,
            bitmap,
            old,
            bits: bits.cast(),
            surface,
            width,
            height,
        })
    }
    unsafe fn present(&mut self, hwnd: HWND, monitor: &Display) -> anyhow::Result<()> {
        self.surface.flush();
        {
            let pixels = self.surface.data()?;
            std::ptr::copy_nonoverlapping(pixels.as_ptr(), self.bits, pixels.len());
        }
        let point = POINT {
            x: monitor.left,
            y: monitor.top,
        };
        let size = SIZE {
            cx: self.width,
            cy: self.height,
        };
        let origin = POINT { x: 0, y: 0 };
        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };
        anyhow::ensure!(
            UpdateLayeredWindow(
                hwnd,
                null_mut(),
                &point,
                &size,
                self.dc,
                &origin,
                0,
                &blend,
                ULW_ALPHA
            ) != 0,
            "UpdateLayeredWindow failed"
        );
        Ok(())
    }
    unsafe fn snapshot(&self, monitor: &Display) -> anyhow::Result<ImageSurface> {
        let screen = GetDC(null_mut());
        anyhow::ensure!(!screen.is_null(), "Desktop DC unavailable");
        let copied = BitBlt(
            self.dc,
            0,
            0,
            self.width,
            self.height,
            screen,
            monitor.left,
            monitor.top,
            SRCCOPY | CAPTUREBLT,
        );
        ReleaseDC(null_mut(), screen);
        anyhow::ensure!(copied != 0, "Desktop snapshot failed");
        let length = self.width as usize * self.height as usize * 4;
        let mut bytes = std::slice::from_raw_parts(self.bits, length).to_vec();
        for pixel in bytes.chunks_exact_mut(4) {
            pixel[3] = 255;
        }
        Ok(ImageSurface::create_for_data(
            bytes,
            Format::ARgb32,
            self.width,
            self.height,
            self.width * 4,
        )?)
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            SelectObject(self.dc, self.old);
            DeleteObject(self.bitmap);
            DeleteDC(self.dc);
        }
    }
}

struct Runtime {
    controller: Arc<Controller>,
    hwnd: HWND,
    scene: Scene,
    mode: Mode,
    monitor: Option<Display>,
    buffer: Option<Buffer>,
    snapshot: Option<ImageSurface>,
    clock: Instant,
    enabled: bool,
    keys: bool,
    transition: bool,
    serial: u64,
    buttons: u8,
    zoom: f64,
}
impl Runtime {
    unsafe fn stop(&mut self) {
        self.enabled = false;
        self.transition = true;
        ReleaseCapture();
        ShowWindow(self.hwnd, SW_HIDE);
        self.transition = false;
        if self.keys {
            for id in 100..104 {
                UnregisterHotKey(self.hwnd, id);
            }
            self.keys = false;
        }
        self.mode = Mode::Normal;
        self.snapshot = None;
        self.buffer = None;
        self.scene = Scene::default();
    }
    unsafe fn keys(&mut self) -> anyhow::Result<()> {
        if self.keys {
            return Ok(());
        }
        self.keys = true;
        for (id, key) in [
            (100, u32::from(b'D')),
            (101, u32::from(b'B')),
            (102, u32::from(b'Z')),
            (103, u32::from(VK_F12)),
        ] {
            anyhow::ensure!(
                RegisterHotKey(self.hwnd, id, MOD_CONTROL | MOD_ALT | MOD_NOREPEAT, key) != 0,
                "Shortcut unavailable (Ctrl+Alt+D/B/Z/F12); close the conflicting application"
            );
        }
        Ok(())
    }
    unsafe fn monitor(&mut self) -> anyhow::Result<()> {
        let mut point: POINT = std::mem::zeroed();
        anyhow::ensure!(GetCursorPos(&mut point) != 0, "Cursor unavailable");
        let found = windows_displays::enumerate()?
            .into_iter()
            .find(|d| {
                point.x >= d.left && point.x < d.right && point.y >= d.top && point.y < d.bottom
            })
            .ok_or_else(|| anyhow::anyhow!("Monitor unavailable"))?;
        if self.monitor.as_ref() != Some(&found) || self.buffer.is_none() {
            self.buffer = Some(Buffer::new(found.width() as i32, found.height() as i32)?);
            self.scene = Scene::default();
            self.monitor = Some(found);
        }
        Ok(())
    }
    unsafe fn mode(&mut self, mode: Mode) -> anyhow::Result<()> {
        self.transition = true;
        ReleaseCapture();
        ShowWindow(self.hwnd, SW_HIDE);
        self.transition = false;
        self.snapshot = None;
        self.scene.erase();
        self.monitor()?;
        self.mode = mode;
        self.zoom = self.controller.bridge.appearance().presentation.zoom;
        if mode == Mode::Zoom {
            self.snapshot = Some(
                self.buffer
                    .as_ref()
                    .unwrap()
                    .snapshot(self.monitor.as_ref().unwrap())?,
            );
        }
        let ex = WS_EX_LAYERED
            | WS_EX_TOOLWINDOW
            | WS_EX_TOPMOST
            | if mode == Mode::Normal {
                WS_EX_TRANSPARENT | WS_EX_NOACTIVATE
            } else {
                0
            };
        SetWindowLongPtrW(self.hwnd, GWL_EXSTYLE, ex as isize);
        self.frame()?;
        ShowWindow(
            self.hwnd,
            if mode == Mode::Normal {
                SW_SHOWNOACTIVATE
            } else {
                SW_SHOW
            },
        );
        if mode != Mode::Normal {
            anyhow::ensure!(
                SetForegroundWindow(self.hwnd) != 0,
                "Cannot focus presentation tools"
            );
            SetFocus(self.hwnd);
        }
        *self.controller.status.lock().unwrap() = format!("{:?}", mode);
        Ok(())
    }
    unsafe fn frame(&mut self) -> anyhow::Result<()> {
        if self.mode == Mode::Normal {
            self.monitor()?;
        }
        let m = self.monitor.as_ref().unwrap();
        let mut point: POINT = std::mem::zeroed();
        anyhow::ensure!(GetCursorPos(&mut point) != 0, "Cursor unavailable");
        let a = self.controller.bridge.appearance();
        let t = self.clock.elapsed().as_secs_f64();
        self.scene.move_to(
            Point {
                x: f64::from(point.x - m.left),
                y: f64::from(point.y - m.top),
            },
            t,
            a.presentation.shake,
        );
        let mut buttons = 0;
        for (i, key) in [VK_LBUTTON, VK_RBUTTON, VK_MBUTTON].iter().enumerate() {
            if GetAsyncKeyState(i32::from(*key)) < 0 {
                buttons |= 1 << i;
            }
        }
        if buttons & !self.buttons != 0 {
            self.scene.click(t);
        }
        self.buttons = buttons;
        let buffer = self.buffer.as_mut().unwrap();
        let c = Context::new(&buffer.surface)?;
        c.set_operator(Operator::Source);
        c.set_source_rgba(
            0.0,
            0.0,
            0.0,
            if self.mode == Mode::Annotate {
                1.0 / 255.0
            } else {
                0.0
            },
        );
        c.paint()?;
        c.set_operator(Operator::Over);
        if let Some(snapshot) = &self.snapshot {
            let origin = zoom_origin(
                self.scene.pointer,
                f64::from(buffer.width),
                f64::from(buffer.height),
                self.zoom,
            );
            c.save()?;
            c.scale(self.zoom, self.zoom);
            c.set_source_surface(snapshot, -origin.x, -origin.y)?;
            c.paint()?;
            c.restore()?;
        }
        drawing::draw(
            &c,
            (f64::from(buffer.width), f64::from(buffer.height)),
            &self.scene,
            &a,
            self.mode,
            t,
        );
        drop(c);
        buffer.present(self.hwnd, m)
    }
    unsafe fn error(&mut self, error: anyhow::Error) {
        self.stop();
        self.controller.fail(error.to_string());
    }
}
unsafe extern "system" fn procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let pointer = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Runtime;
    if pointer.is_null() {
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }
    let r = &mut *pointer;
    match msg {
        WM_CLOSE => {
            r.stop();
            r.controller.enable(false);
            0
        }
        WM_WTSSESSION_CHANGE => {
            if wparam == WTS_SESSION_LOCK as usize
                || wparam == WTS_CONSOLE_DISCONNECT as usize
                || wparam == WTS_REMOTE_DISCONNECT as usize
            {
                r.stop();
                r.controller.enable(false);
                r.controller.bridge.stop();
            }
            0
        }
        WM_DISPLAYCHANGE => {
            r.stop();
            r.controller.enable(false);
            0
        }
        WM_KILLFOCUS => {
            if !r.transition && r.mode != Mode::Normal {
                if let Err(e) = r.mode(Mode::Normal) {
                    r.error(e);
                }
            }
            0
        }
        WM_HOTKEY => {
            if wparam == 103 {
                r.stop();
                r.controller.enable(false);
                r.controller.bridge.stop();
            } else if r.enabled {
                let mode = match wparam {
                    100 => Mode::Annotate,
                    101 => Mode::Whiteboard,
                    _ => Mode::Zoom,
                };
                if let Err(e) = r.mode(mode) {
                    r.error(e);
                }
            }
            0
        }
        WM_KEYDOWN | WM_SYSKEYDOWN if r.mode != Mode::Normal => {
            let key = wparam as u16;
            if key == VK_ESCAPE {
                if let Err(e) = r.mode(Mode::Normal) {
                    r.error(e);
                }
            } else if key == VK_BACK {
                r.scene.erase();
            } else if key == u16::from(b'Z') && GetAsyncKeyState(i32::from(VK_CONTROL)) < 0 {
                r.scene.undo();
            } else if r.mode == Mode::Zoom {
                if key == VK_ADD || key == VK_OEM_PLUS {
                    r.zoom = (r.zoom + 0.25).min(8.0);
                }
                if key == VK_SUBTRACT || key == VK_OEM_MINUS {
                    r.zoom = (r.zoom - 0.25).max(1.25);
                }
            }
            0
        }
        WM_LBUTTONDOWN if matches!(r.mode, Mode::Annotate | Mode::Whiteboard) => {
            r.scene.pointer = Point {
                x: f64::from(lparam as i16),
                y: f64::from((lparam >> 16) as i16),
            };
            r.scene
                .begin_stroke(&r.controller.bridge.appearance().presentation);
            SetCapture(hwnd);
            0
        }
        WM_LBUTTONUP => {
            r.scene.drawing = false;
            ReleaseCapture();
            0
        }
        WM_CAPTURECHANGED => {
            r.scene.drawing = false;
            0
        }
        WM_MOUSEMOVE if r.scene.drawing => {
            r.scene.move_to(
                Point {
                    x: f64::from(lparam as i16),
                    y: f64::from((lparam >> 16) as i16),
                },
                r.clock.elapsed().as_secs_f64(),
                false,
            );
            0
        }
        WM_MOUSEWHEEL if r.mode == Mode::Zoom => {
            let delta = ((wparam >> 16) as i16) as f64;
            r.zoom = (r.zoom + delta / 120.0 * 0.25).clamp(1.25, 8.0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
pub(super) fn run(controller: &Arc<Controller>) {
    unsafe {
        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        let instance = GetModuleHandleW(null());
        let class: Vec<u16> = "KeycastBridgePresentation\0".encode_utf16().collect();
        let wc = WNDCLASSW {
            lpfnWndProc: Some(procedure),
            hInstance: instance,
            lpszClassName: class.as_ptr(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            ..std::mem::zeroed()
        };
        if RegisterClassW(&wc) == 0 {
            controller.fail("Cannot register presentation window");
            return;
        }
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT,
            class.as_ptr(),
            class.as_ptr(),
            WS_POPUP,
            0,
            0,
            1,
            1,
            null_mut(),
            null_mut(),
            instance,
            null(),
        );
        if hwnd.is_null() {
            controller.fail("Cannot create presentation window");
            UnregisterClassW(class.as_ptr(), instance);
            return;
        }
        let mut runtime = Box::new(Runtime {
            controller: controller.clone(),
            hwnd,
            scene: Scene::default(),
            mode: Mode::Normal,
            monitor: None,
            buffer: None,
            snapshot: None,
            clock: Instant::now(),
            enabled: false,
            keys: false,
            transition: false,
            serial: 0,
            buttons: 0,
            zoom: 2.0,
        });
        SetWindowLongPtrW(
            hwnd,
            GWLP_USERDATA,
            (&mut *runtime as *mut Runtime) as isize,
        );
        if WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION) == 0 {
            controller.fail("Cannot monitor session lock");
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            DestroyWindow(hwnd);
            UnregisterClassW(class.as_ptr(), instance);
            return;
        }
        while !controller.stopped() {
            let mut message: MSG = std::mem::zeroed();
            while PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            let request = controller.request.lock().unwrap().clone();
            if !request.enabled {
                if runtime.enabled {
                    runtime.stop();
                }
            } else if request.serial != runtime.serial {
                runtime.serial = request.serial;
                runtime.enabled = true;
                if let Err(e) = runtime.keys().and_then(|()| runtime.mode(request.mode)) {
                    runtime.error(e);
                }
            }
            if runtime.enabled {
                if let Err(e) = runtime.frame() {
                    runtime.error(e);
                }
            }
            std::thread::sleep(Duration::from_millis(33));
        }
        runtime.stop();
        WTSUnRegisterSessionNotification(hwnd);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        DestroyWindow(hwnd);
        UnregisterClassW(class.as_ptr(), instance);
    }
}
