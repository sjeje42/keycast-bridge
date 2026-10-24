//! Explicitly enabled desktop presentation tools. Runtime state is never persisted.
use crate::{presentation::Mode, server::Bridge};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

#[cfg(any(windows, test))]
mod drawing;
#[cfg(target_os = "linux")]
mod gnome;
#[cfg(windows)]
mod windows;

#[derive(Clone, serde::Serialize)]
pub(super) struct Request {
    pub enabled: bool,
    pub mode: Mode,
    pub serial: u64,
}
pub struct Controller {
    request: Mutex<Request>,
    status: Mutex<String>,
    shutdown: AtomicBool,
    finished: AtomicBool,
    bridge: Arc<Bridge>,
}
impl Controller {
    pub fn new(bridge: Arc<Bridge>) -> Arc<Self> {
        let this = Arc::new(Self {
            request: Mutex::new(Request {
                enabled: false,
                mode: Mode::Normal,
                serial: 0,
            }),
            status: Mutex::new(String::new()),
            shutdown: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            bridge,
        });
        let worker = this.clone();
        std::thread::spawn(move || {
            #[cfg(target_os = "linux")]
            gnome::run(&worker);
            #[cfg(windows)]
            windows::run(&worker);
            worker.finished.store(true, Ordering::Release);
        });
        this
    }
    pub fn enable(&self, enabled: bool) {
        let mut r = self.request.lock().unwrap();
        r.enabled = enabled;
        r.mode = Mode::Normal;
        r.serial += 1;
    }
    pub fn mode(&self, mode: Mode) {
        let mut r = self.request.lock().unwrap();
        if r.enabled {
            r.mode = mode;
            r.serial += 1;
        }
    }
    pub fn enabled(&self) -> bool {
        self.request.lock().unwrap().enabled
    }
    pub fn status(&self) -> String {
        self.status.lock().unwrap().clone()
    }
    pub fn shutdown(&self) {
        self.enable(false);
        self.shutdown.store(true, Ordering::Release);
        // Give the backend time to release grabs and erase its in-memory snapshot.
        for _ in 0..100 {
            if self.finished.load(Ordering::Acquire) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    fn stopped(&self) -> bool {
        self.shutdown.load(Ordering::Acquire)
    }
    fn fail(&self, message: impl Into<String>) {
        *self.status.lock().unwrap() = message.into();
        self.enable(false);
    }
}
