#[cfg(target_os = "linux")]
pub mod capture;
pub mod model;
pub mod server;
#[cfg(windows)]
pub mod windows_capture;

#[cfg(target_os = "linux")]
pub mod linux_broker;
#[cfg(target_os = "linux")]
pub mod linux_devices;

pub mod displays;
#[cfg(windows)]
pub mod windows_displays;
