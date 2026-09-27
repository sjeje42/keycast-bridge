//! Privileged opener and unprivileged event reader, joined by an inherited
//! private AF_UNIX socketpair. No filesystem socket or external clients.
use crate::{
    capture::{Action, Normalizer},
    linux_devices::{self, Info},
    model::Event,
};
use anyhow::{Context, Result};
use evdev::{Device, EventType, KeyCode};
use nix::{
    sys::{
        socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags},
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::{fork, ForkResult, Pid},
};
use std::{
    collections::HashMap,
    io::{BufRead, IoSlice, IoSliceMut, Write},
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        unix::{fs::MetadataExt, net::UnixDatagram},
    },
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(serde::Serialize, serde::Deserialize)]
enum Notice {
    Add(Info),
    Remove(String),
}
fn emit(event: Event) -> Result<()> {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer(&mut out, &event)?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}
fn send(socket: &UnixDatagram, notice: &Notice, fd: Option<i32>) -> Result<()> {
    let bytes = serde_json::to_vec(notice)?;
    let fds = fd.into_iter().collect::<Vec<_>>();
    let controls = if fds.is_empty() {
        vec![]
    } else {
        vec![ControlMessage::ScmRights(&fds)]
    };
    sendmsg::<()>(
        socket.as_raw_fd(),
        &[IoSlice::new(&bytes)],
        &controls,
        MsgFlags::MSG_DONTWAIT | MsgFlags::MSG_NOSIGNAL,
        None,
    )?;
    Ok(())
}
fn receive(socket: &UnixDatagram) -> Result<Option<(Notice, Option<OwnedFd>)>> {
    let mut bytes = [0u8; 8192];
    let mut controls = nix::cmsg_space!([i32; 1]);
    let (count, flags, descriptors) = {
        let mut iov = [IoSliceMut::new(&mut bytes)];
        let msg = match recvmsg::<()>(
            socket.as_raw_fd(),
            &mut iov,
            Some(&mut controls),
            MsgFlags::MSG_DONTWAIT | MsgFlags::MSG_CMSG_CLOEXEC,
        ) {
            Ok(m) => m,
            Err(nix::errno::Errno::EAGAIN) => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        let mut fds = Vec::new();
        for c in msg.cmsgs()? {
            if let ControlMessageOwned::ScmRights(raw) = c {
                for fd in raw {
                    fds.push(unsafe { OwnedFd::from_raw_fd(fd) });
                }
            }
        }
        (msg.bytes, msg.flags, fds)
    };
    anyhow::ensure!(
        !flags.intersects(MsgFlags::MSG_TRUNC | MsgFlags::MSG_CTRUNC),
        "Truncated broker message"
    );
    let notice: Notice = serde_json::from_slice(&bytes[..count])?;
    anyhow::ensure!(
        descriptors.len() == usize::from(matches!(notice, Notice::Add(_))),
        "Invalid descriptor count"
    );
    Ok(Some((notice, descriptors.into_iter().next())))
}

pub fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    anyhow::ensure!(
        args.len() == 4,
        "Expected selection, layout, shortcuts|all, mouse|no-mouse"
    );
    anyhow::ensure!(
        ["fr", "us", "gb", "de"].contains(&args[1].as_str()),
        "Invalid layout"
    );
    anyhow::ensure!(
        ["shortcuts", "all"].contains(&args[2].as_str()),
        "Invalid mode"
    );
    anyhow::ensure!(
        ["mouse", "no-mouse"].contains(&args[3].as_str()),
        "Invalid mouse mode"
    );
    anyhow::ensure!(unsafe { libc::geteuid() } == 0, "Launch through pkexec");
    let uid: u32 = std::env::var("PKEXEC_UID")
        .context("Launch through pkexec")?
        .parse()?;
    anyhow::ensure!(uid != 0, "Refusing root session");
    let gid = unsafe {
        let pw = libc::getpwuid(uid);
        anyhow::ensure!(!pw.is_null(), "Unknown user");
        (*pw).pw_gid
    };
    // Nothing has started threads before fork.
    let (broker, reader) = UnixDatagram::pair()?;
    let parent_pid = unsafe { libc::getpid() };
    match unsafe { fork()? } {
        ForkResult::Child => {
            drop(broker);
            let result = (|| -> Result<()> {
                unsafe {
                    anyhow::ensure!(
                        libc::setgroups(0, std::ptr::null()) == 0
                            && libc::setgid(gid) == 0
                            && libc::setuid(uid) == 0,
                        "Cannot drop privileges"
                    );
                    anyhow::ensure!(
                        libc::getuid() == uid && libc::geteuid() == uid,
                        "Privilege drop failed"
                    );
                    anyhow::ensure!(
                        libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM, 0, 0, 0) == 0
                            && libc::getppid() == parent_pid,
                        "Broker exited during startup"
                    );
                    anyhow::ensure!(
                        libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) == 0,
                        "Cannot restrict privileges"
                    );
                }
                read_devices(reader, &args[1], args[2] == "all")
            })();
            if let Err(e) = &result {
                eprintln!("Capture: {e:#}");
            }
            unsafe {
                libc::_exit(if result.is_ok() { 0 } else { 1 });
            }
        }
        ForkResult::Parent { child } => {
            drop(reader);
            // Only the reader inherits application IPC; broker never reads keys.
            unsafe {
                libc::close(0);
                libc::close(1);
            }
            let result = broker_loop(&broker, child, &args[0], args[3] == "mouse");
            if result.is_err() {
                unsafe {
                    libc::kill(child.as_raw(), libc::SIGTERM);
                }
                let _ = waitpid(child, None);
            }
            result
        }
    }
}

fn same_keyboard(wanted: &Info, info: &Info) -> bool {
    if wanted.name != info.name || wanted.vendor != info.vendor || wanted.product != info.product {
        return false;
    }
    match (&wanted.serial, &info.serial) {
        (Some(a), Some(b)) => a == b,
        _ => wanted.physical.is_some() && wanted.physical == info.physical,
    }
}

fn broker_loop(socket: &UnixDatagram, child: Pid, selection: &str, mouse: bool) -> Result<()> {
    // Start listening before enumeration to avoid losing an arrival during startup.
    let monitor = udev::MonitorBuilder::new()?
        .match_subsystem("input")?
        .listen()?;
    let mut devices: HashMap<String, Info> = HashMap::new();
    let initial = linux_devices::enumerate()?;
    let selected = if selection == "all" {
        None
    } else {
        let ids: Vec<String> = serde_json::from_str(selection).context("Invalid selection")?;
        anyhow::ensure!(
            !ids.is_empty() && ids.len() <= 128,
            "Invalid keyboard selection"
        );
        let mut selected = Vec::new();
        for id in ids {
            selected.push(
                initial
                    .iter()
                    .find(|(_, i)| i.keyboard && i.id == id)
                    .context("Selected keyboard not present")?
                    .1
                    .clone(),
            );
        }
        Some(selected)
    };
    let keyboard_allowed = |info: &Info, initial: bool| {
        info.keyboard
            && selected.as_ref().is_none_or(|list| {
                list.iter()
                    .any(|wanted| (initial && wanted.id == info.id) || same_keyboard(wanted, info))
            })
    };
    let accepts =
        |info: &Info, initial: bool| keyboard_allowed(info, initial) || (mouse && info.mouse);
    let open = |device: &udev::Device, info: &Info, initial: bool| -> Result<()> {
        let mut info = info.clone();
        info.keyboard = keyboard_allowed(&info, initial);
        info.mouse = mouse && info.mouse;
        use std::os::unix::fs::OpenOptionsExt;
        let file = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NONBLOCK | libc::O_NOFOLLOW)
            .open(&info.path)?;
        anyhow::ensure!(
            device.devnum() == Some(file.metadata()?.rdev()),
            "Device changed during open"
        );
        let input = Device::from_fd(file.into())?;
        let keys = input.supported_keys();
        anyhow::ensure!(
            keys.is_some_and(|k| (info.keyboard
                && k.contains(KeyCode::KEY_A)
                && k.contains(KeyCode::KEY_ENTER))
                || (mouse && info.mouse && k.contains(KeyCode::BTN_LEFT))),
            "Not an authorized input device"
        );
        send(socket, &Notice::Add(info), Some(input.as_raw_fd()))
    };
    for (device, info) in initial {
        if accepts(&info, true) && open(&device, &info, true).is_ok() {
            devices.insert(info.id.clone(), info);
        }
    }
    loop {
        match waitpid(child, Some(WaitPidFlag::WNOHANG))? {
            WaitStatus::StillAlive => (),
            WaitStatus::Exited(_, 0) => return Ok(()),
            _ => anyhow::bail!("Capture reader stopped unexpectedly"),
        }
        for event in monitor.iter() {
            let id = event.syspath().to_string_lossy().into_owned();
            if event.event_type() == udev::EventType::Remove {
                if devices.remove(&id).is_some() {
                    send(socket, &Notice::Remove(id), None)?;
                }
            } else if matches!(
                event.event_type(),
                udev::EventType::Add | udev::EventType::Change
            ) {
                if let Some(info) = linux_devices::info(&event) {
                    if !devices.contains_key(&id)
                        && accepts(&info, false)
                        && open(&event, &info, false).is_ok()
                    {
                        devices.insert(id, info);
                    }
                }
            }
        }
        let mut fd = libc::pollfd {
            fd: monitor.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        unsafe {
            libc::poll(&mut fd, 1, 100);
        }
    }
}
struct Input {
    info: Info,
    device: Device,
    normalizer: Normalizer,
}
fn read_devices(socket: UnixDatagram, layout: &str, all: bool) -> Result<()> {
    let alive = Arc::new(AtomicBool::new(true));
    let last = Arc::new(std::sync::Mutex::new(Instant::now()));
    let (a, l) = (alive.clone(), last.clone());
    std::thread::spawn(move || {
        let mut input = std::io::BufReader::new(std::io::stdin());
        let mut line = String::new();
        loop {
            line.clear();
            match input.read_line(&mut line) {
                Ok(_) if line == "ping\n" => *l.lock().unwrap() = Instant::now(),
                _ => break,
            }
        }
        a.store(false, Ordering::Release);
    });
    let epoll_fd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
    anyhow::ensure!(epoll_fd >= 0, "Cannot create epoll");
    let epoll = unsafe { OwnedFd::from_raw_fd(epoll_fd) };
    let mut inputs: HashMap<i32, Input> = HashMap::new();
    let mut held: HashMap<i32, Vec<String>> = HashMap::new();
    let mut socket_event = libc::epoll_event {
        events: libc::EPOLLIN as u32,
        u64: socket.as_raw_fd() as u64,
    };
    anyhow::ensure!(
        unsafe {
            libc::epoll_ctl(
                epoll.as_raw_fd(),
                libc::EPOLL_CTL_ADD,
                socket.as_raw_fd(),
                &mut socket_event,
            )
        } == 0,
        "Cannot watch broker"
    );
    emit(Event::Ready)?;
    emit(Event::DeviceStatus {
        message: "En attente de périphériques / Waiting for devices".into(),
    })?;
    while alive.load(Ordering::Acquire) && last.lock().unwrap().elapsed() < Duration::from_secs(3) {
        let mut events = [libc::epoll_event { events: 0, u64: 0 }; 64];
        let count = unsafe { libc::epoll_wait(epoll.as_raw_fd(), events.as_mut_ptr(), 64, 100) };
        if count < 0 {
            if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            anyhow::bail!("epoll failed");
        }
        for event in &events[..count as usize] {
            let fd = event.u64 as i32;
            if fd == socket.as_raw_fd() {
                while let Some((notice, owned)) = receive(&socket)? {
                    match notice {
                        Notice::Add(info) => {
                            let device = Device::from_fd(owned.context("Missing descriptor")?)?;
                            device.set_nonblocking(true)?;
                            let fd = device.as_raw_fd();
                            let mut ev = libc::epoll_event {
                                events: libc::EPOLLIN as u32,
                                u64: fd as u64,
                            };
                            anyhow::ensure!(
                                unsafe {
                                    libc::epoll_ctl(
                                        epoll.as_raw_fd(),
                                        libc::EPOLL_CTL_ADD,
                                        fd,
                                        &mut ev,
                                    )
                                } == 0,
                                "Cannot watch input"
                            );
                            emit(Event::DeviceStatus {
                                message: format!("Connecté / Connected: {}", info.name),
                            })?;
                            inputs.insert(
                                fd,
                                Input {
                                    info,
                                    device,
                                    normalizer: Normalizer::new(layout, all)?,
                                },
                            );
                        }
                        Notice::Remove(id) => {
                            if let Some(fd) = inputs
                                .iter()
                                .find(|(_, i)| i.info.id == id)
                                .map(|(fd, _)| *fd)
                            {
                                remove(&mut inputs, &mut held, fd, epoll.as_raw_fd())?;
                            }
                        }
                    }
                }
            } else if let Some(input) = inputs.get_mut(&fd) {
                let result = input
                    .device
                    .fetch_events()
                    .map(|events| events.collect::<Vec<_>>());
                match result {
                    Ok(events) => {
                        for event in events {
                            if event.event_type() == EventType::KEY {
                                let code = event.code();
                                if input.info.mouse
                                    && (272..=274).contains(&code)
                                    && event.value() != 2
                                {
                                    emit(Event::Mouse {
                                        button: (code - 271) as u8,
                                        pressed: event.value() == 1,
                                        x: None,
                                        y: None,
                                    })?;
                                } else if input.info.keyboard {
                                    let before = input.normalizer.modifiers();
                                    let action = input.normalizer.event(code, event.value());
                                    let keys = input.normalizer.modifiers();
                                    if keys != before {
                                        held.insert(fd, keys);
                                        emit(held_modifiers(&held))?;
                                    }
                                    match action {
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
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
                    Err(_) => remove(&mut inputs, &mut held, fd, epoll.as_raw_fd())?,
                }
            }
        }
    }
    emit(Event::Clear)?;
    Ok(())
}
fn held_modifiers(held: &HashMap<i32, Vec<String>>) -> Event {
    Event::Modifiers {
        keys: ["Ctrl", "Alt", "Super", "Shift", "AltGr"]
            .into_iter()
            .filter(|name| held.values().any(|keys| keys.iter().any(|key| key == name)))
            .map(str::to_owned)
            .collect(),
    }
}

fn remove(
    inputs: &mut HashMap<i32, Input>,
    held: &mut HashMap<i32, Vec<String>>,
    fd: i32,
    epoll: i32,
) -> Result<()> {
    unsafe {
        libc::epoll_ctl(epoll, libc::EPOLL_CTL_DEL, fd, std::ptr::null_mut());
    }
    if let Some(input) = inputs.remove(&fd) {
        // Drop per-device modifier state, including keys held while unplugging.
        held.remove(&fd);
        emit(Event::Clear)?;
        emit(held_modifiers(held))?;
        emit(Event::DeviceStatus {
            message: format!(
                "Déconnecté / Disconnected: {} — {} actif(s) / active",
                input.info.name,
                inputs.len()
            ),
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn removal_preserves_modifiers_held_on_another_keyboard() {
        let mut held = HashMap::from([
            (1, vec!["Ctrl".into()]),
            (2, vec!["Ctrl".into(), "Shift".into()]),
        ]);
        let keys = |held: &HashMap<i32, Vec<String>>| match held_modifiers(held) {
            Event::Modifiers { keys } => keys,
            _ => unreachable!(),
        };
        assert_eq!(keys(&held), ["Ctrl", "Shift"]);
        held.remove(&1);
        assert_eq!(keys(&held), ["Ctrl", "Shift"]);
        held.remove(&2);
        assert!(keys(&held).is_empty());
    }
    fn info() -> Info {
        Info {
            id: "/sys/input/event4".into(),
            path: "/dev/input/event4".into(),
            name: "Keyboard".into(),
            keyboard: true,
            mouse: false,
            vendor: Some("1234".into()),
            product: Some("5678".into()),
            serial: Some("unique-123".into()),
            physical: Some("usb-port-1".into()),
        }
    }
    #[test]
    fn identity_survives_event_number_changes_but_not_replacement() {
        let a = info();
        let mut b = a.clone();
        b.id = "/sys/input/event11".into();
        b.path = "/dev/input/event11".into();
        assert!(same_keyboard(&a, &b));
        b.serial = Some("different-device".into());
        assert!(!same_keyboard(&a, &b));
        let mut a = a;
        a.serial = None;
        b.serial = None;
        assert!(same_keyboard(&a, &b));
        b.physical = Some("other-port".into());
        assert!(!same_keyboard(&a, &b));
    }
    #[test]
    fn private_socket_transfers_owned_descriptor_and_removal() {
        let (a, b) = UnixDatagram::pair().unwrap();
        let file = std::fs::File::open("/dev/null").unwrap();
        send(&a, &Notice::Add(info()), Some(file.as_raw_fd())).unwrap();
        drop(file);
        let (notice, fd) = receive(&b).unwrap().unwrap();
        assert!(matches!(notice, Notice::Add(_)));
        let mut file = std::fs::File::from(fd.unwrap());
        assert_eq!(std::io::Read::read(&mut file, &mut [0u8; 1]).unwrap(), 0);
        send(&a, &Notice::Remove(info().id), None).unwrap();
        let (notice, fd) = receive(&b).unwrap().unwrap();
        assert!(matches!(notice, Notice::Remove(_)) && fd.is_none());
        assert!(receive(&b).unwrap().is_none());
    }
    #[test]
    fn add_without_descriptor_is_rejected() {
        let (a, b) = UnixDatagram::pair().unwrap();
        send(&a, &Notice::Add(info()), None).unwrap();
        assert!(receive(&b).is_err());
    }
}
