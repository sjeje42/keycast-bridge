//! Enumerate in the same physical coordinate space as MSLLHOOKSTRUCT::pt.
use crate::displays::Display;
use windows_sys::Win32::{
    Foundation::{LPARAM, RECT},
    Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW},
    UI::HiDpi::{SetThreadDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
};

pub fn enumerate() -> anyhow::Result<Vec<Display>> {
    unsafe extern "system" fn collect(
        monitor: HMONITOR,
        _: HDC,
        _: *mut RECT,
        data: LPARAM,
    ) -> i32 {
        let list = &mut *(data as *mut Vec<Display>);
        let mut info: MONITORINFOEXW = std::mem::zeroed();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
        if GetMonitorInfoW(monitor, &mut info.monitorInfo) == 0 {
            return 0;
        }
        let length = info
            .szDevice
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(info.szDevice.len());
        let r = info.monitorInfo.rcMonitor;
        let display = Display {
            id: String::from_utf16_lossy(&info.szDevice[..length]),
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
            primary: info.monitorInfo.dwFlags & 1 != 0, // MONITORINFOF_PRIMARY
        };
        if display.width() > 0 && display.height() > 0 {
            list.push(display);
        }
        1
    }
    // Restore GTK's thread awareness after querying; never change process-wide DPI here.
    let previous =
        unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    anyhow::ensure!(
        !previous.is_null(),
        "Cannot query physical display coordinates"
    );
    let mut displays: Vec<Display> = Vec::new();
    let ok = unsafe {
        let ok = EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null(),
            Some(collect),
            &mut displays as *mut _ as LPARAM,
        );
        SetThreadDpiAwarenessContext(previous);
        ok
    };
    anyhow::ensure!(ok != 0, "Cannot enumerate monitors");
    displays.sort_by(|a, b| b.primary.cmp(&a.primary).then_with(|| a.id.cmp(&b.id)));
    Ok(displays)
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_enumeration_returns_a_usable_primary_monitor() {
        let list = super::enumerate().unwrap();
        let primary = crate::displays::selected(&list, None).expect("No primary monitor");
        assert!(!primary.id.is_empty());
        assert!(primary.project(primary.left, primary.top).is_some());
    }
}
