use gtk4::{
    glib, prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton,
    ComboBoxText, Entry, Label, Orientation, SpinButton,
};
use keycast_bridge::{
    model::Event,
    server::{self, Bridge, PORT},
};
use std::{
    io::{BufRead, Write},
    process::{Command, Stdio},
    sync::Arc,
    time::Duration,
};

const HELPER: &str = "/usr/local/libexec/keycast-bridge-capture";
fn tr(fr: bool, french: &'static str, english: &'static str) -> &'static str {
    if fr {
        french
    } else {
        english
    }
}

fn start(state: Arc<Bridge>, device: String, layout: String, all: bool) {
    state.stop();
    let generation = {
        let mut inner = state.inner.lock().unwrap();
        inner.status = "authorizing";
        inner.session
    };
    std::thread::spawn(move || {
        let result = (|| -> anyhow::Result<()> {
            let mut child = Command::new("/usr/bin/pkexec")
                .args([
                    HELPER,
                    &device,
                    &layout,
                    if all { "all" } else { "shortcuts" },
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?;
            let mut input = child.stdin.take().unwrap();
            let output = child.stdout.take().unwrap();
            let heartbeat_state = state.clone();
            let heartbeat = std::thread::spawn(move || {
                loop {
                    if heartbeat_state.inner.lock().unwrap().session != generation {
                        break;
                    }
                    if input
                        .write_all(b"ping\n")
                        .and_then(|_| input.flush())
                        .is_err()
                    {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(500));
                }
                // Closing stdin is the unprivileged shutdown mechanism, even during pkexec.
            });
            for line in std::io::BufReader::new(output).lines() {
                let event: Event = serde_json::from_str(&line?)?;
                let mut inner = state.inner.lock().unwrap();
                if inner.session != generation {
                    continue;
                }
                match event {
                    Event::Ready => inner.status = "capturing",
                    Event::Key { .. } => {
                        let _ = state.tx.send(event);
                    }
                    Event::Clear => {
                        inner.status = "stopped";
                        let _ = state.tx.send(Event::Clear);
                    }
                    _ => (),
                }
            }
            let status = child.wait()?;
            {
                let mut inner = state.inner.lock().unwrap();
                if inner.session == generation {
                    inner.session += 1;
                    inner.status = if status.success() { "stopped" } else { "error" };
                    let _ = state.tx.send(Event::Clear);
                }
            }
            let _ = heartbeat.join();
            Ok(())
        })();
        if result.is_err() {
            let mut inner = state.inner.lock().unwrap();
            if inner.session == generation {
                inner.session += 1;
                inner.status = "error";
                let _ = state.tx.send(Event::Clear);
            }
        }
    });
}

fn populate(devices: &ComboBoxText) {
    devices.remove_all();
    if let Ok(entries) = std::fs::read_dir("/sys/class/input") {
        let mut choices = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.starts_with("event") {
                continue;
            }
            let label =
                std::fs::read_to_string(entry.path().join("device/name")).unwrap_or_default();
            choices.push((
                format!("/dev/input/{name}"),
                format!("{} — {name}", label.trim()),
            ));
        }
        choices.sort();
        for (id, label) in choices {
            devices.append(Some(&id), &label);
        }
    }
    devices.set_active(Some(0));
}
fn main() -> anyhow::Result<()> {
    anyhow::ensure!(
        unsafe { libc::geteuid() } != 0,
        "Do not launch the application as root"
    );
    let state = Bridge::new();
    // Bind synchronously: don't show a usable URL if another instance owns the port.
    let listener = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, PORT))?;
    listener.set_nonblocking(true)?;
    let server_state = state.clone();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::from_std(listener).unwrap();
            if server::serve(server_state.clone(), listener).await.is_err() {
                server_state.stop();
            }
        });
    });
    let app = Application::builder()
        .application_id("fr.jeromelab.KeycastBridge")
        .build();
    let ui_state = state.clone();
    app.connect_activate(move |app| build_ui(app, ui_state.clone()));
    app.run();
    state.stop();
    Ok(())
}
fn build_ui(app: &Application, state: Arc<Bridge>) {
    if let Some(window) = app.active_window() {
        window.present();
        return;
    }
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Keycast Bridge")
        .default_width(680)
        .default_height(650)
        .build();
    let root = GtkBox::new(Orientation::Vertical, 14);
    for set in [
        GtkBox::set_margin_top,
        GtkBox::set_margin_bottom,
        GtkBox::set_margin_start,
        GtkBox::set_margin_end,
    ] {
        set(&root, 24);
    }
    let title = Label::new(Some("Keycast Bridge"));
    title.add_css_class("title-1");
    title.set_xalign(0.0);
    root.append(&title);
    let language = ComboBoxText::new();
    language.append(Some("fr"), "Français");
    language.append(Some("en"), "English");
    language.set_active_id(Some(
        if std::env::var("LANG").unwrap_or_default().starts_with("fr") {
            "fr"
        } else {
            "en"
        },
    ));
    root.append(&language);
    let status = Label::new(None);
    status.set_xalign(0.0);
    status.add_css_class("heading");
    root.append(&status);
    let keyboard_label = Label::new(None);
    keyboard_label.set_xalign(0.0);
    root.append(&keyboard_label);
    let devices = ComboBoxText::new();
    populate(&devices);
    root.append(&devices);
    let refresh = Button::new();
    root.append(&refresh);
    {
        let devices = devices.clone();
        refresh.connect_clicked(move |_| populate(&devices));
    }
    let layout_label = Label::new(None);
    layout_label.set_xalign(0.0);
    root.append(&layout_label);
    let layout = ComboBoxText::new();
    for (id, label) in [
        ("fr", "AZERTY — France"),
        ("us", "QWERTY — US"),
        ("gb", "QWERTY — UK"),
        ("de", "QWERTZ — DE"),
    ] {
        layout.append(Some(id), label);
    }
    layout.set_active(Some(0));
    root.append(&layout);
    let all = CheckButton::new();
    root.append(&all);
    let actions = GtkBox::new(Orientation::Horizontal, 8);
    let start_button = Button::new();
    start_button.add_css_class("suggested-action");
    let stop_button = Button::new();
    stop_button.add_css_class("destructive-action");
    let demo = Button::new();
    for button in [&start_button, &stop_button, &demo] {
        actions.append(button);
    }
    root.append(&actions);
    {
        let (s, d, l, a) = (state.clone(), devices.clone(), layout.clone(), all.clone());
        start_button.connect_clicked(move |_| {
            if let (Some(device), Some(layout)) = (d.active_id(), l.active_id()) {
                start(
                    s.clone(),
                    device.to_string(),
                    layout.to_string(),
                    a.is_active(),
                );
            }
        });
    }
    {
        let s = state.clone();
        stop_button.connect_clicked(move |_| s.stop());
    }
    {
        let s = state.clone();
        demo.connect_clicked(move |_| {
            s.stop();
            let _ = s.tx.send(Event::Key {
                label: "Ctrl + Shift + V".into(),
            });
        });
    }
    let obs = Label::new(None);
    obs.set_xalign(0.0);
    root.append(&obs);
    let url = Entry::new();
    url.set_editable(false);
    url.set_text(&state.url());
    root.append(&url);
    let copy = Button::new();
    root.append(&copy);
    {
        let s = state.clone();
        copy.connect_clicked(move |b| b.clipboard().set_text(&s.url()));
    }
    let preview = Button::new();
    root.append(&preview);
    {
        let s = state.clone();
        preview.connect_clicked(move |_| {
            let _ = Command::new("xdg-open").arg(s.url()).spawn();
        });
    }
    let appearance = GtkBox::new(Orientation::Horizontal, 8);
    let size_label = Label::new(None);
    let size = SpinButton::with_range(20.0, 96.0, 2.0);
    size.set_value(40.0);
    let duration_label = Label::new(None);
    let duration = SpinButton::with_range(300.0, 5000.0, 100.0);
    duration.set_value(1800.0);
    let dark = CheckButton::new();
    dark.set_active(true);
    appearance.append(&size_label);
    appearance.append(&size);
    appearance.append(&duration_label);
    appearance.append(&duration);
    appearance.append(&dark);
    root.append(&appearance);
    {
        let (s, d, k) = (state.clone(), duration.clone(), dark.clone());
        size.connect_value_changed(move |v| {
            s.config(
                v.value_as_int() as u32,
                d.value_as_int() as u32,
                k.is_active(),
            )
        });
    }
    {
        let (s, z, k) = (state.clone(), size.clone(), dark.clone());
        duration.connect_value_changed(move |v| {
            s.config(
                z.value_as_int() as u32,
                v.value_as_int() as u32,
                k.is_active(),
            )
        });
    }
    {
        let (s, z, d) = (state.clone(), size.clone(), duration.clone());
        dark.connect_toggled(move |k| {
            s.config(
                z.value_as_int() as u32,
                d.value_as_int() as u32,
                k.is_active(),
            )
        });
    }
    let notice = Label::new(None);
    notice.set_wrap(true);
    notice.set_xalign(0.0);
    root.append(&notice);
    let weak = window.downgrade();
    let s = state.clone();
    glib::timeout_add_local(Duration::from_millis(150), move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let fr = language.active_id().is_some_and(|s| s == "fr");
        let current = s.inner.lock().unwrap().status;
        let active = matches!(current, "authorizing" | "capturing");
        status.set_text(match current {
            "capturing" => tr(fr, "● Capture active", "● Capture active"),
            "authorizing" => tr(fr, "Autorisation en cours…", "Waiting for authorization…"),
            "error" => tr(
                fr,
                "Échec : vérifier le clavier, l’installation et l’autorisation.",
                "Failed: check keyboard, installation and authorization.",
            ),
            _ => tr(fr, "○ Capture arrêtée", "○ Capture stopped"),
        });
        keyboard_label.set_text(tr(
            fr,
            "Clavier (les périphériques non clavier seront refusés)",
            "Keyboard (non-keyboard devices will be rejected)",
        ));
        refresh.set_label(tr(fr, "Actualiser les périphériques", "Refresh devices"));
        layout_label.set_text(tr(
            fr,
            "Disposition — doit correspondre à celle de GNOME",
            "Layout — must match your GNOME layout",
        ));
        all.set_label(Some(tr(
            fr,
            "Afficher aussi les touches de texte (risque de divulgation)",
            "Also show text keys (may reveal private information)",
        )));
        start_button.set_label(tr(fr, "Démarrer", "Start"));
        stop_button.set_label(tr(fr, "Arrêter", "Stop"));
        demo.set_label(tr(fr, "Tester le rendu", "Test overlay"));
        start_button.set_sensitive(!active && devices.active_id().is_some());
        for widget in [&devices, &layout] {
            widget.set_sensitive(!active);
        }
        all.set_sensitive(!active);
        refresh.set_sensitive(!active);
        obs.set_text(tr(
            fr,
            "OBS → Source Navigateur → URL (1920 × 1080)",
            "OBS → Browser source → URL (1920 × 1080)",
        ));
        copy.set_label(tr(fr, "Copier l’URL OBS", "Copy OBS URL"));
        preview.set_label(tr(fr, "Ouvrir l’aperçu", "Open preview"));
        size_label.set_text(tr(fr, "Taille", "Size"));
        duration_label.set_text(tr(fr, "Durée (ms)", "Duration (ms)"));
        dark.set_label(Some(tr(fr, "Sombre", "Dark")));
        notice.set_text(tr(fr, "Ctrl + Alt + F12 : arrêt immédiat. Aucun historique. Pas de détection des mots de passe. L’URL change à chaque lancement : la recopier dans OBS.", "Ctrl + Alt + F12: stop immediately. No history. No password-field detection. The URL changes on each launch: update it in OBS."));
        glib::ControlFlow::Continue
    });
    window.set_child(Some(&root));
    window.present();
}
