//! The only privileged entry point. No network, files written, or subprocesses.
use anyhow::{bail, Context, Result};
use evdev::{Device, EventType, KeyCode};
use keycast_bridge::{
    capture::{Action, Normalizer},
    model::Event,
};
use std::{
    io::{BufRead, Write},
    os::unix::fs::FileTypeExt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

fn emit(event: Event) -> Result<()> {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer(&mut out, &event)?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    anyhow::ensure!(
        args.len() == 3,
        "Expected event device, layout and shortcuts|all"
    );
    anyhow::ensure!(
        matches!(args[2].as_str(), "shortcuts" | "all"),
        "Invalid mode"
    );
    let mut normalizer = Normalizer::new(&args[1], args[2] == "all")?;
    let path = std::fs::canonicalize(&args[0])?;
    anyhow::ensure!(
        path.parent() == Some(std::path::Path::new("/dev/input")),
        "Invalid device directory"
    );
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    anyhow::ensure!(
        name.strip_prefix("event")
            .is_some_and(|s| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())),
        "Invalid event name"
    );
    anyhow::ensure!(
        std::fs::metadata(&path)?.file_type().is_char_device(),
        "Not a character device"
    );
    let mut device = Device::open(&path).context("Cannot open keyboard")?;
    anyhow::ensure!(
        device
            .supported_keys()
            .is_some_and(|keys| keys.contains(KeyCode::KEY_A) && keys.contains(KeyCode::KEY_ENTER)),
        "Not a keyboard"
    );
    device.set_nonblocking(true)?;
    // Keep only the already-open descriptor. Never run the event loop as root.
    unsafe {
        if libc::geteuid() == 0 {
            let uid: libc::uid_t = std::env::var("PKEXEC_UID")
                .context("Launch with pkexec")?
                .parse()?;
            anyhow::ensure!(uid != 0, "Refusing root session");
            let pw = libc::getpwuid(uid);
            anyhow::ensure!(!pw.is_null(), "Unknown user");
            let gid = (*pw).pw_gid;
            if libc::setgroups(0, std::ptr::null()) != 0
                || libc::setgid(gid) != 0
                || libc::setuid(uid) != 0
            {
                bail!("Cannot drop privileges");
            }
            anyhow::ensure!(
                libc::geteuid() == uid && libc::getuid() == uid,
                "Privilege drop failed"
            );
        }
    }
    let alive = Arc::new(AtomicBool::new(true));
    let last = Arc::new(std::sync::Mutex::new(Instant::now()));
    let (a, l) = (alive.clone(), last.clone());
    std::thread::spawn(move || {
        // Parent sends a small heartbeat. EOF, invalid input or silence ends capture.
        let mut input = std::io::BufReader::new(std::io::stdin());
        let mut line = String::new();
        loop {
            line.clear();
            match input.read_line(&mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) if line == "ping\n" => *l.lock().unwrap() = Instant::now(),
                _ => break,
            }
        }
        a.store(false, Ordering::Release);
    });
    emit(Event::Ready)?;
    while alive.load(Ordering::Acquire) && last.lock().unwrap().elapsed() < Duration::from_secs(3) {
        match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    if event.event_type() == EventType::KEY {
                        match normalizer.event(event.code(), event.value()) {
                            Action::Label(label) => emit(Event::Key { label })?,
                            Action::Stop => {
                                emit(Event::Clear)?;
                                return Ok(());
                            }
                            Action::Ignore => (),
                        }
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(8))
            }
            Err(e) => return Err(e.into()),
        }
    }
    emit(Event::Clear)?;
    Ok(())
}
