//! Windows capture on a dedicated message-pump thread. No elevation or disk I/O.
use crate::{model::Event, server::Bridge};
use std::{
    cell::{Cell, RefCell},
    sync::{mpsc, Arc},
    time::Duration,
};
use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        HiDpi::{SetThreadDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
        Input::KeyboardAndMouse::{GetKeyState, GetKeyboardLayout, ToUnicodeEx},
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetForegroundWindow, GetSystemMetrics,
            GetWindowThreadProcessId, PeekMessageW, SetWindowsHookExW, TranslateMessage,
            UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT, PM_REMOVE,
            SM_CXSCREEN, SM_CYSCREEN, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYUP, WM_LBUTTONDOWN,
            WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYUP,
        },
    },
};

#[derive(Clone, Copy)]
struct Raw {
    vk: u32,
    scan: u32,
    up: bool,
    layout: isize,
}
enum Input {
    Key(Raw),
    Mouse {
        button: u8,
        pressed: bool,
        x: f64,
        y: f64,
    },
}
thread_local! {
    static EVENTS: RefCell<Option<mpsc::SyncSender<Input>>> = const { RefCell::new(None) };
    static OVERFLOW: Cell<bool> = const { Cell::new(false) };
}

unsafe extern "system" fn hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == 0 {
        let key = &*(lparam as *const KBDLLHOOKSTRUCT);
        let thread = GetWindowThreadProcessId(GetForegroundWindow(), std::ptr::null_mut());
        let raw = Raw {
            vk: key.vkCode,
            scan: key.scanCode,
            up: matches!(wparam as u32, WM_KEYUP | WM_SYSKEYUP),
            layout: GetKeyboardLayout(thread) as isize,
        };
        EVENTS.with(|events| {
            if let Some(sender) = events.borrow().as_ref() {
                if sender.try_send(Input::Key(raw)).is_err() {
                    OVERFLOW.set(true);
                }
            }
        });
    }
    // Never consume or block the user's keystroke.
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == 0 {
        let button = match wparam as u32 {
            WM_LBUTTONDOWN | WM_LBUTTONUP => 1,
            WM_RBUTTONDOWN | WM_RBUTTONUP => 2,
            WM_MBUTTONDOWN | WM_MBUTTONUP => 3,
            _ => 0,
        };
        if button != 0 {
            let point = &*(lparam as *const MSLLHOOKSTRUCT);
            let input = Input::Mouse {
                button,
                pressed: matches!(
                    wparam as u32,
                    WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN
                ),
                x: f64::from(point.pt.x) / f64::from(GetSystemMetrics(SM_CXSCREEN).max(1)),
                y: f64::from(point.pt.y) / f64::from(GetSystemMetrics(SM_CYSCREEN).max(1)),
            };
            EVENTS.with(|events| {
                if let Some(sender) = events.borrow().as_ref() {
                    if sender.try_send(input).is_err() {
                        OVERFLOW.set(true);
                    }
                }
            });
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

struct Hook(HHOOK);
impl Drop for Hook {
    fn drop(&mut self) {
        unsafe {
            UnhookWindowsHookEx(self.0);
        }
        EVENTS.with(|events| *events.borrow_mut() = None);
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Ignore,
    Stop,
    Label(String),
}
struct Normalizer {
    down: [bool; 256],
    caps: bool,
    all: bool,
}
impl Normalizer {
    fn new(all: bool) -> Self {
        Self {
            down: [false; 256],
            caps: unsafe { GetKeyState(0x14) & 1 != 0 },
            all,
        }
    }
    fn event(&mut self, raw: Raw) -> Action {
        let vk = raw.vk as usize;
        if vk >= 256 {
            return Action::Ignore;
        }
        if raw.up {
            self.down[vk] = false;
            return Action::Ignore;
        }
        if self.down[vk] {
            return Action::Ignore;
        }
        self.down[vk] = true;
        if vk == 0x14 {
            self.caps = !self.caps;
        }
        let ctrl = self.down[0xa2] || self.down[0xa3];
        let altgr = self.down[0xa5];
        let alt = self.down[0xa4];
        let shift = self.down[0xa0] || self.down[0xa1];
        let win = self.down[0x5b] || self.down[0x5c];
        if vk == 0x7b && ctrl && alt && !altgr {
            return Action::Stop;
        }
        if matches!(vk, 0x10..=0x14 | 0x5b..=0x5c | 0xa0..=0xa5 | 0x90..=0x91) {
            return Action::Ignore;
        }
        let shortcut = !altgr && (ctrl || alt || win);
        let named = match vk {
            0x08 => Some("Backspace".into()),
            0x09 => Some("Tab".into()),
            0x0d => Some("Enter".into()),
            0x1b => Some("Esc".into()),
            0x20 => Some("Space".into()),
            0x21 => Some("PageUp".into()),
            0x22 => Some("PageDown".into()),
            0x23 => Some("End".into()),
            0x24 => Some("Home".into()),
            0x25 => Some("←".into()),
            0x26 => Some("↑".into()),
            0x27 => Some("→".into()),
            0x28 => Some("↓".into()),
            0x2d => Some("Insert".into()),
            0x2e => Some("Delete".into()),
            0x70..=0x87 => Some(format!("F{}", vk - 0x6f)),
            _ => None,
        };
        if !self.all && !shortcut && (named.is_none() || vk == 0x20) {
            return Action::Ignore;
        }
        let label = named.unwrap_or_else(|| {
            let mut state = [0u8; 256];
            // Shortcuts use the base symbol; text uses Shift/AltGr/Caps Lock.
            if !shortcut {
                if shift {
                    state[0x10] = 0x80;
                }
                if altgr {
                    state[0x11] = 0x80;
                    state[0x12] = 0x80;
                    state[0xa5] = 0x80;
                }
                state[0x14] = u8::from(self.caps);
            }
            let mut text = [0u16; 16];
            // Flag 4 prevents modifying Windows' dead-key composition state.
            let len = unsafe {
                ToUnicodeEx(
                    raw.vk,
                    raw.scan,
                    state.as_ptr(),
                    text.as_mut_ptr(),
                    text.len() as i32,
                    4,
                    raw.layout as _,
                )
            };
            if len <= 0 {
                return String::new();
            }
            let text = String::from_utf16_lossy(&text[..(len as usize).min(text.len())]);
            if shortcut {
                text.to_uppercase()
            } else {
                text
            }
        });
        if label.is_empty() || label.chars().any(char::is_control) {
            return Action::Ignore;
        }
        let mut parts = Vec::new();
        // Windows synthesizes Ctrl for AltGr: never display that as Ctrl+Alt.
        if ctrl && !altgr {
            parts.push("Ctrl".to_owned());
        }
        if alt && !altgr {
            parts.push("Alt".to_owned());
        }
        if win {
            parts.push("Win".to_owned());
        }
        if shift {
            parts.push("Shift".to_owned());
        }
        if altgr {
            parts.push("AltGr".to_owned());
        }
        parts.push(label);
        Action::Label(parts.join(" + "))
    }
}

pub fn start(state: Arc<Bridge>, all: bool, mouse: bool) {
    state.stop();
    let generation = {
        let mut inner = state.inner.lock().unwrap();
        inner.status = "authorizing";
        inner.session
    };
    std::thread::spawn(move || {
        let result = run(&state, generation, all, mouse);
        let mut inner = state.inner.lock().unwrap();
        if inner.session == generation {
            inner.session += 1;
            inner.status = if result.is_ok() { "stopped" } else { "error" };
            let _ = state.tx.send(Event::Clear);
        }
    });
}

fn run(state: &Bridge, generation: u64, all: bool, mouse: bool) -> anyhow::Result<()> {
    unsafe {
        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    let (sender, receiver) = mpsc::sync_channel(256);
    EVENTS.with(|events| *events.borrow_mut() = Some(sender));
    OVERFLOW.set(false);
    let handle = unsafe {
        SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(hook),
            GetModuleHandleW(std::ptr::null()),
            0,
        )
    };
    if handle.is_null() {
        EVENTS.with(|events| *events.borrow_mut() = None);
        anyhow::bail!(
            "Cannot install Windows keyboard hook: {}",
            std::io::Error::last_os_error()
        );
    }
    let _hook = Hook(handle);
    let _mouse_hook = if mouse {
        let handle = unsafe {
            SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook),
                GetModuleHandleW(std::ptr::null()),
                0,
            )
        };
        anyhow::ensure!(!handle.is_null(), "Cannot install mouse hook");
        Some(Hook(handle))
    } else {
        None
    };
    let mut normalizer = Normalizer::new(all);
    {
        let mut inner = state.inner.lock().unwrap();
        if inner.session != generation {
            return Ok(());
        }
        inner.status = "capturing";
    }
    loop {
        if state.inner.lock().unwrap().session != generation {
            return Ok(());
        }
        let mut msg: MSG = unsafe { std::mem::zeroed() };
        // Bound work each iteration so a message flood cannot prevent Stop.
        for _ in 0..256 {
            if unsafe { PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) } == 0 {
                break;
            }
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        anyhow::ensure!(!OVERFLOW.get(), "Keyboard event queue overflow");
        for input in receiver.try_iter().take(256) {
            let raw = match input {
                Input::Key(raw) => raw,
                Input::Mouse {
                    button,
                    pressed,
                    x,
                    y,
                } => {
                    let inner = state.inner.lock().unwrap();
                    if inner.session != generation {
                        return Ok(());
                    }
                    let inside = (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y);
                    let _ = state.tx.send(Event::Mouse {
                        button,
                        pressed,
                        x: inside.then_some(x),
                        y: inside.then_some(y),
                    });
                    continue;
                }
            };
            match normalizer.event(raw) {
                Action::Stop => return Ok(()),
                Action::Label(label) => {
                    let inner = state.inner.lock().unwrap();
                    if inner.session != generation {
                        return Ok(());
                    }
                    let _ = state.tx.send(Event::Key { label });
                }
                Action::Ignore => (),
            }
        }
        std::thread::sleep(Duration::from_millis(8));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        LoadKeyboardLayoutW, UnloadKeyboardLayout,
    };
    fn raw(vk: u32, layout: isize) -> Raw {
        Raw {
            vk,
            scan: 0,
            up: false,
            layout,
        }
    }
    #[test]
    fn text_repeat_release_and_emergency_stop() {
        let mut n = Normalizer::new(false);
        assert_eq!(n.event(raw(0x41, 0)), Action::Ignore);
        n.event(raw(0xa2, 0));
        n.event(raw(0xa4, 0));
        assert_eq!(n.event(raw(0x7b, 0)), Action::Stop);
        assert_eq!(n.event(raw(0x7b, 0)), Action::Ignore);
        n.event(Raw {
            up: true,
            ..raw(0xa2, 0)
        });
        n.event(Raw {
            up: true,
            ..raw(0x7b, 0)
        });
        assert_eq!(n.event(raw(0x7b, 0)), Action::Label("Alt + F12".into()));
    }
    #[test]
    fn altgr_does_not_leak_as_ctrl_alt() {
        let mut n = Normalizer::new(false);
        n.event(raw(0xa2, 0));
        n.event(raw(0xa5, 0));
        assert_eq!(n.event(raw(0x45, 0)), Action::Ignore);
        assert_eq!(n.event(raw(0x20, 0)), Action::Ignore);
    }
    #[test]
    fn french_and_us_layouts_translate_shortcuts() {
        for id in ["0000040c", "00000409"] {
            let wide: Vec<u16> = id.encode_utf16().chain(Some(0)).collect();
            let layout = unsafe { LoadKeyboardLayoutW(wide.as_ptr(), 0) };
            assert!(!layout.is_null(), "Layout unavailable: {id}");
            let mut n = Normalizer::new(false);
            n.event(raw(0xa2, layout as isize));
            // VK_A is already layout-dependent when delivered by Windows.
            assert_eq!(
                n.event(raw(0x41, layout as isize)),
                Action::Label("Ctrl + A".into())
            );
            unsafe {
                UnloadKeyboardLayout(layout);
            }
        }
    }
}
