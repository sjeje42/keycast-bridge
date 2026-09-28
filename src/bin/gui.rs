#![cfg_attr(windows, windows_subsystem = "windows")]

#[path = "gui/appearance.rs"]
mod appearance_controls;
#[path = "gui/browser.rs"]
mod browser;

use gtk4::{
    glib, prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton,
    ComboBoxText, Entry, Label, Orientation, ScrolledWindow, SpinButton,
};
use keycast_bridge::{
    model::Event,
    server::{self, Bridge, PORT},
};
#[cfg(target_os = "linux")]
use std::{
    io::{BufRead, Write},
    process::{Command, Stdio},
};
use std::{sync::Arc, time::Duration};

#[cfg(target_os = "linux")]
const HELPER: &str = match option_env!("KEYCAST_HELPER_PATH") {
    Some(path) => path,
    None => "/usr/local/libexec/keycast-bridge-capture",
};
fn tr(fr: bool, french: &'static str, english: &'static str) -> &'static str {
    if fr {
        french
    } else {
        english
    }
}

#[cfg(target_os = "linux")]
fn start(state: Arc<Bridge>, device: String, layout: String, all: bool, mouse: bool) {
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
                    if mouse { "mouse" } else { "no-mouse" },
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
                    Event::Key { .. } | Event::Mouse { .. } => {
                        let _ = state.tx.send(event);
                    }
                    Event::Modifiers { keys } => {
                        inner.modifiers = keys.clone();
                        let _ = state.tx.send(Event::Modifiers { keys });
                    }
                    Event::DeviceStatus { message } => inner.device_status = message,
                    Event::Clear => {
                        inner.modifiers.clear();
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
                    inner.modifiers.clear();
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
                inner.modifiers.clear();
                let _ = state.tx.send(Event::Clear);
            }
        }
    });
}

type Choices = std::rc::Rc<std::cell::RefCell<Vec<(String, CheckButton)>>>;
#[cfg(target_os = "linux")]
fn populate(devices: &ComboBoxText, list: &GtkBox, choices: &Choices) {
    let previous: Vec<String> = choices
        .borrow()
        .iter()
        .filter(|(_, c)| c.is_active())
        .map(|(id, _)| id.clone())
        .collect();
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
    choices.borrow_mut().clear();
    if devices.active_id().is_none() {
        devices.append(
            Some("all"),
            "Tous les claviers (automatique) / All keyboards (automatic)",
        );
        devices.append(
            Some("selected"),
            "Claviers sélectionnés / Selected keyboards",
        );
        devices.set_active(Some(0));
    }
    if let Ok(found) = keycast_bridge::linux_devices::enumerate() {
        for (_, info) in found.into_iter().filter(|(_, i)| i.keyboard) {
            let check = CheckButton::with_label(&info.name);
            check.set_tooltip_text(info.physical.as_deref());
            check.set_active(previous.contains(&info.id));
            list.append(&check);
            choices.borrow_mut().push((info.id, check));
        }
    }
}

#[cfg(windows)]
fn start(state: Arc<Bridge>, _device: String, _layout: String, all: bool, mouse: bool) {
    keycast_bridge::windows_capture::start(state, all, mouse);
}

#[cfg(windows)]
fn populate(devices: &ComboBoxText, _list: &GtkBox, _choices: &Choices) {
    devices.remove_all();
    devices.append(
        Some("windows"),
        "Windows — tous les claviers / all keyboards",
    );
    devices.set_active(Some(0));
}

fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
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
    if std::env::var("KEYCAST_SMOKE_TEST").as_deref() == Ok("1") {
        let smoke_app = app.clone();
        glib::timeout_add_local_once(Duration::from_secs(2), move || smoke_app.quit());
    }
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
        .default_width(640)
        .default_height(520)
        .build();
    fit_window(&window, 640, 520);
    let settings = gtk4::Window::builder()
        .transient_for(&window)
        .destroy_with_parent(true)
        .hide_on_close(true)
        .build();
    fit_window(&settings, 720, 640);
    let settings_root = GtkBox::new(Orientation::Vertical, 8);
    let tabs = gtk4::Notebook::new();
    tabs.set_vexpand(true);
    let style_page = GtkBox::new(Orientation::Vertical, 8);
    let format_page = GtkBox::new(Orientation::Vertical, 10);
    let capture_page = GtkBox::new(Orientation::Vertical, 8);
    let style_tab = Label::new(None);
    let format_tab = Label::new(None);
    let capture_tab = Label::new(None);
    for (page, label) in [
        (&style_page, &style_tab),
        (&format_page, &format_tab),
        (&capture_page, &capture_tab),
    ] {
        for set in [
            GtkBox::set_margin_top,
            GtkBox::set_margin_bottom,
            GtkBox::set_margin_start,
            GtkBox::set_margin_end,
        ] {
            set(page, 16);
        }
        let scroll = ScrolledWindow::builder()
            .child(page)
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();
        tabs.append_page(&scroll, Some(label));
    }
    settings_root.append(&tabs);
    let settings_footer = GtkBox::new(Orientation::Horizontal, 8);
    settings_footer.set_margin_start(16);
    settings_footer.set_margin_end(16);
    settings_footer.set_margin_bottom(12);
    let guide = Button::new();
    let close_settings = Button::new();
    settings_footer.append(&guide);
    settings_footer.append(&close_settings);
    settings_root.append(&settings_footer);
    settings.set_child(Some(&settings_root));
    {
        let settings = settings.clone();
        close_settings.connect_clicked(move |_| settings.hide());
    }
    let root = GtkBox::new(Orientation::Vertical, 10);
    for set in [
        GtkBox::set_margin_top,
        GtkBox::set_margin_bottom,
        GtkBox::set_margin_start,
        GtkBox::set_margin_end,
    ] {
        set(&root, 16);
    }
    let title = Label::new(Some("Keycast Bridge"));
    title.add_css_class("title-1");
    title.set_xalign(0.0);
    title.set_hexpand(true);
    title.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    let header = GtkBox::new(Orientation::Horizontal, 8);
    header.append(&title);
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
    header.append(&language);
    let gear = Button::from_icon_name("preferences-system-symbolic");
    header.append(&gear);
    root.append(&header);
    {
        let settings = settings.clone();
        gear.connect_clicked(move |_| settings.present());
    }
    {
        let state = state.clone();
        let language = language.clone();
        let settings = settings.clone();
        guide.connect_clicked(move |_| {
            let fr = language.active_id().is_some_and(|id| id == "fr");
            browser::open(&settings, &state.help_url(fr), fr);
        });
    }
    let status = Label::new(None);
    status.set_xalign(0.0);
    status.add_css_class("heading");
    root.append(&status);
    let keyboard_label = Label::new(None);
    keyboard_label.set_xalign(0.0);
    keyboard_label.set_wrap(true);
    root.append(&keyboard_label);
    let devices = ComboBoxText::new();
    let choices: Choices = Default::default();
    let device_list = GtkBox::new(Orientation::Vertical, 4);
    populate(&devices, &device_list, &choices);
    capture_page.append(&devices);
    let device_scroll = ScrolledWindow::builder()
        .child(&device_list)
        .max_content_height(130)
        .propagate_natural_height(true)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .build();
    capture_page.append(&device_scroll);
    let refresh = Button::new();
    capture_page.append(&refresh);
    {
        let devices = devices.clone();
        let list = device_list.clone();
        let choices = choices.clone();
        refresh.connect_clicked(move |_| populate(&devices, &list, &choices));
    }
    let layout_label = Label::new(None);
    layout_label.set_xalign(0.0);
    capture_page.append(&layout_label);
    let layout = ComboBoxText::new();
    for (id, label) in [
        ("fr", "AZERTY — France"),
        ("us", "QWERTY — US"),
        ("gb", "QWERTY — UK"),
        ("de", "QWERTZ — DE"),
    ] {
        layout.append(Some(id), label);
    }
    #[cfg(windows)]
    {
        layout.remove_all();
        layout.append(Some("auto"), "Automatique / Automatic — Windows");
    }
    layout.set_active(Some(0));
    capture_page.append(&layout);
    let all = CheckButton::new();
    capture_page.append(&all);
    let mouse = CheckButton::new();
    let mouse_row = GtkBox::new(Orientation::Horizontal, 8);
    mouse_row.append(&mouse);
    let halo = CheckButton::new();
    halo.set_visible(cfg!(windows));
    mouse_row.append(&halo);
    root.append(&mouse_row);
    let screen_summary = Label::new(None);
    screen_summary.set_xalign(0.0);
    screen_summary.set_wrap(true);
    root.append(&screen_summary);
    {
        let s = state.clone();
        let mouse = mouse.clone();
        halo.connect_toggled(move |v| {
            if v.is_active() {
                mouse.set_active(true);
            }
            s.halo(v.is_active());
        });
    }
    {
        let halo = halo.clone();
        mouse.connect_toggled(move |v| {
            if !v.is_active() {
                halo.set_active(false);
            }
        });
    }
    #[cfg(windows)]
    display_controls(
        &capture_page,
        state.clone(),
        language.clone(),
        mouse.clone(),
        halo.clone(),
    );
    let device_status = Label::new(None);
    device_status.set_xalign(0.0);
    device_status.set_wrap(true);
    root.append(&device_status);
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
        let mouse = mouse.clone();
        let choices = choices.clone();
        start_button.connect_clicked(move |_| {
            if let (Some(device), Some(layout)) = (d.active_id(), l.active_id()) {
                let selection = if device == "selected" {
                    let ids: Vec<String> = choices
                        .borrow()
                        .iter()
                        .filter(|(_, c)| c.is_active())
                        .map(|(id, _)| id.clone())
                        .collect();
                    if ids.is_empty() {
                        return;
                    }
                    serde_json::to_string(&ids).unwrap()
                } else {
                    device.to_string()
                };
                start(
                    s.clone(),
                    selection,
                    layout.to_string(),
                    a.is_active(),
                    mouse.is_active(),
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
    let obs_actions = GtkBox::new(Orientation::Horizontal, 8);
    obs_actions.append(&copy);
    {
        let s = state.clone();
        copy.connect_clicked(move |b| b.clipboard().set_text(&s.url()));
    }
    let preview = Button::new();
    obs_actions.append(&preview);
    root.append(&obs_actions);
    {
        let s = state.clone();
        let window = window.clone();
        let language = language.clone();
        preview.connect_clicked(move |_| {
            let fr = language.active_id().is_some_and(|id| id == "fr");
            browser::open(&window, &s.url(), fr);
        });
    }
    let appearance = GtkBox::new(Orientation::Horizontal, 8);
    let size_label = Label::new(None);
    let size = SpinButton::with_range(20.0, 96.0, 2.0);
    size.set_value(40.0);
    let duration_label = Label::new(None);
    let duration = SpinButton::with_range(300.0, 5000.0, 100.0);
    duration.set_value(1800.0);
    appearance.append(&size_label);
    appearance.append(&size);
    appearance.append(&duration_label);
    appearance.append(&duration);
    format_page.append(&appearance);
    {
        let (s, d) = (state.clone(), duration.clone());
        size.connect_value_changed(move |v| {
            s.config(v.value_as_int() as u32, d.value_as_int() as u32)
        });
    }
    {
        let (s, z) = (state.clone(), size.clone());
        duration.connect_value_changed(move |v| {
            s.config(z.value_as_int() as u32, v.value_as_int() as u32)
        });
    }
    appearance_controls::build(&style_page, state.clone(), language.clone());
    appearance_controls::canvas_controls(&format_page, state.clone(), language.clone());
    let notice = Label::new(None);
    notice.set_wrap(true);
    notice.set_xalign(0.0);
    root.append(&notice);
    let weak = window.downgrade();
    let s = state.clone();
    let settings_for_smoke = settings.clone();
    let gear_for_smoke = gear.clone();
    let close_for_smoke = close_settings.clone();
    let mouse_for_smoke = mouse.clone();
    let halo_for_smoke = halo.clone();
    glib::timeout_add_local(Duration::from_millis(150), move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let fr = language.active_id().is_some_and(|s| s == "fr");
        let current = s.inner.lock().unwrap().status;
        settings.set_title(Some(tr(
            fr,
            "Paramètres — Keycast Bridge",
            "Settings — Keycast Bridge",
        )));
        gear.set_tooltip_text(Some(tr(fr, "Paramètres", "Settings")));
        gear.update_property(&[gtk4::accessible::Property::Label(tr(
            fr,
            "Paramètres",
            "Settings",
        ))]);
        style_tab.set_text(tr(fr, "Position et couleurs", "Position and colors"));
        format_tab.set_text(tr(fr, "Format et taille", "Canvas and size"));
        capture_tab.set_text(tr(fr, "Capture", "Capture"));
        guide.set_label(tr(fr, "Guide complet", "Complete guide"));
        close_settings.set_label(tr(fr, "Fermer", "Close"));
        let active = matches!(current, "authorizing" | "capturing");
        status.set_text(match current {
            "capturing" => tr(fr, "● Capture active", "● Capture active"),
            "authorizing" => tr(fr, "Autorisation en cours…", "Waiting for authorization…"),
            "error" => tr(
                fr,
                if cfg!(windows) {
                    "Échec de la capture Windows. Arrêter puis réessayer."
                } else {
                    "Échec : vérifier le clavier, l’installation et l’autorisation."
                },
                if cfg!(windows) {
                    "Windows capture failed. Stop and try again."
                } else {
                    "Failed: check keyboard, installation and authorization."
                },
            ),
            _ => tr(fr, "○ Capture arrêtée", "○ Capture stopped"),
        });
        keyboard_label.set_text(tr(
            fr,
            if cfg!(windows) {
                "Claviers Windows — disposition automatique"
            } else {
                "Claviers — sélection et disposition dans les Paramètres"
            },
            if cfg!(windows) {
                "Windows keyboards — automatic layout"
            } else {
                "Keyboards — selection and layout in Settings"
            },
        ));
        refresh.set_label(tr(fr, "Actualiser les périphériques", "Refresh devices"));
        layout_label.set_text(tr(
            fr,
            if cfg!(windows) {
                "Disposition — suit la fenêtre active"
            } else {
                "Disposition — doit correspondre à celle de GNOME"
            },
            if cfg!(windows) {
                "Layout — follows the active window"
            } else {
                "Layout — must match your GNOME layout"
            },
        ));
        all.set_label(Some(tr(
            fr,
            "Afficher aussi les touches de texte (risque de divulgation)",
            "Also show text keys (may reveal private information)",
        )));
        start_button.set_label(tr(fr, "Démarrer", "Start"));
        stop_button.set_label(tr(fr, "Arrêter", "Stop"));
        demo.set_label(tr(fr, "Tester le rendu", "Test overlay"));
        let custom = devices.active_id().is_some_and(|id| id == "selected");
        device_scroll.set_visible(custom);
        device_list.set_sensitive(!active);
        start_button.set_sensitive(
            !active && (!custom || choices.borrow().iter().any(|(_, c)| c.is_active())),
        );
        for widget in [&devices, &layout] {
            widget.set_sensitive(!active);
        }
        if cfg!(windows) {
            devices.set_sensitive(false);
            layout.set_sensitive(false);
            refresh.set_visible(false);
        }
        all.set_sensitive(!active);
        mouse.set_sensitive(!active);
        mouse.set_label(Some(tr(
            fr,
            "Afficher les clics de souris",
            "Show mouse clicks",
        )));
        halo.set_label(Some(tr(fr, "Cercle au clic", "Click ring")));
        halo.set_sensitive(!active || mouse.is_active());
        let message = s.inner.lock().unwrap().device_status.clone();
        device_status.set_text(&message);
        device_status.set_visible(!message.is_empty());
        screen_summary.set_visible(mouse.is_active() && halo.is_active() && cfg!(windows));
        let display = s
            .inner
            .lock()
            .unwrap()
            .halo_display
            .clone()
            .unwrap_or_default();
        screen_summary.set_text(&format!(
            "{} {} — {}",
            tr(fr, "Cercle :", "Ring:"),
            display.trim_start_matches(r"\\.\"),
            tr(
                fr,
                "changer dans Paramètres → Capture",
                "change in Settings → Capture"
            )
        ));
        refresh.set_sensitive(!active);
        let canvas = s.appearance();
        obs.set_wrap(true);
        obs.set_text(&format!(
            "{} — {} × {}",
            tr(
                fr,
                "OBS → Source Navigateur → URL",
                "OBS → Browser source → URL"
            ),
            canvas.canvas_width,
            canvas.canvas_height
        ));
        copy.set_label(tr(fr, "Copier l’URL OBS", "Copy OBS URL"));
        preview.set_label(tr(fr, "Ouvrir l’aperçu", "Open preview"));
        size_label.set_text(tr(fr, "Taille", "Size"));
        duration_label.set_text(tr(fr, "Durée (ms)", "Duration (ms)"));
        notice.set_text(tr(fr, "Ctrl + Alt + F12 : arrêt immédiat. Aucun historique. Pas de détection des mots de passe. L’URL change à chaque lancement : la recopier dans OBS.", "Ctrl + Alt + F12: stop immediately. No history. No password-field detection. The URL changes on each launch: update it in OBS."));
        glib::ControlFlow::Continue
    });
    let scroll = ScrolledWindow::builder()
        .child(&root)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .build();
    window.set_child(Some(&scroll));
    window.present();
    if std::env::var("KEYCAST_SMOKE_TEST").as_deref() == Ok("1") {
        let window = window.clone();
        glib::timeout_add_local_once(Duration::from_millis(650), move || {
            if window.height() >= 480 {
                let adjustment = scroll.vadjustment();
                assert!(
                    adjustment.upper() <= adjustment.page_size() + 1.0,
                    "Main controls require scrolling at normal window size"
                );
            }
            if cfg!(windows) {
                assert!(halo_for_smoke.is_sensitive());
                halo_for_smoke.set_active(true);
                assert!(
                    mouse_for_smoke.is_active(),
                    "Ring did not enable mouse capture"
                );
                mouse_for_smoke.set_active(false);
                assert!(
                    !halo_for_smoke.is_active(),
                    "Ring left active without mouse capture"
                );
            }
            gear_for_smoke.emit_clicked();
            assert!(
                settings_for_smoke.is_visible(),
                "Settings gear did not open the window"
            );
            glib::timeout_add_local_once(Duration::from_millis(250), move || {
                close_for_smoke.emit_clicked();
                assert!(!settings_for_smoke.is_visible(), "Settings did not close");
                assert!(
                    window.is_visible(),
                    "Closing Settings closed the main window"
                );
            });
        });
    }
}

fn fit_window(window: &impl IsA<gtk4::Window>, width: i32, height: i32) {
    let mut bounds = (width, height);
    if let Some(display) = gtk4::gdk::Display::default() {
        let monitors = display.monitors();
        for index in 0..monitors.n_items() {
            if let Some(monitor) = monitors.item(index).and_downcast::<gtk4::gdk::Monitor>() {
                let rect = monitor.geometry();
                if rect.width() > 0 && rect.height() > 0 {
                    bounds.0 = bounds.0.min((rect.width() - 64).max(320));
                    bounds.1 = bounds.1.min((rect.height() - 96).max(300));
                }
            }
        }
    }
    window.set_default_size(bounds.0, bounds.1);
}

#[cfg(windows)]
fn display_controls(
    root: &GtkBox,
    state: Arc<Bridge>,
    language: ComboBoxText,
    mouse: CheckButton,
    halo: CheckButton,
) {
    use keycast_bridge::{displays, windows_displays};
    use std::{cell::Cell, rc::Rc};
    let heading = Label::new(None);
    heading.set_xalign(0.0);
    root.append(&heading);
    let screens = ComboBoxText::new();
    root.append(&screens);
    let hint = Label::new(None);
    hint.set_xalign(0.0);
    hint.set_wrap(true);
    root.append(&hint);
    let rebuilding = Rc::new(Cell::new(false));
    {
        let state = state.clone();
        let rebuilding = rebuilding.clone();
        screens.connect_changed(move |combo| {
            if !rebuilding.get() {
                if let Some(id) = combo.active_id() {
                    state.select_display(id.to_string());
                }
            }
        });
    }
    let weak = root.downgrade();
    let mut previous = None;
    let mut update = move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let fr = language.active_id().is_some_and(|id| id == "fr");
        heading.set_text(tr(
            fr,
            "Écran capturé dans OBS (cercle au clic)",
            "Monitor captured in OBS (click ring)",
        ));
        let result = windows_displays::enumerate();
        let failed = result.is_err();
        let list = result.unwrap_or_default();
        let mut selected = state.inner.lock().unwrap().halo_display.clone();
        if selected.is_none()
            || (list.len() == 1 && selected.as_deref() != Some(list[0].id.as_str()))
        {
            if let Some(display) = displays::selected(&list, selected.as_deref()) {
                selected = Some(display.id.clone());
                state.select_display(display.id.clone());
            }
        }
        let current = (list.clone(), selected.clone(), fr);
        if previous.as_ref() != Some(&current) {
            rebuilding.set(true);
            screens.remove_all();
            for display in &list {
                let marker = if display.primary {
                    tr(fr, " — principal", " — primary")
                } else {
                    ""
                };
                screens.append(
                    Some(&display.id),
                    &format!(
                        "{} — {} × {}{}",
                        display.id.trim_start_matches(r"\\.\"),
                        display.width(),
                        display.height(),
                        marker
                    ),
                );
            }
            if let Some(id) = &selected {
                if !list.iter().any(|d| &d.id == id) {
                    screens.append(
                        Some(id),
                        &format!(
                            "{} — {}",
                            id.trim_start_matches(r"\\.\"),
                            tr(fr, "indisponible", "unavailable")
                        ),
                    );
                }
                screens.set_active_id(Some(id));
            }
            rebuilding.set(false);
            previous = Some(current);
        }
        screens.set_sensitive(mouse.is_active() && halo.is_active() && list.len() > 1);
        screens.set_tooltip_text(Some(tr(
            fr,
            "Avec un seul écran, la sélection est automatique.",
            "With one monitor, selection is automatic.",
        )));
        if failed {
            hint.set_text(tr(fr, "Impossible de détecter les écrans. Le cercle est suspendu ; nouvelle tentative automatique.", "Cannot detect monitors. The ring is suspended; detection retries automatically."));
        } else if let Some(display) = displays::selected(&list, selected.as_deref()) {
            hint.set_text(&format!("{} × {} — {} ({}, {}). {}", display.width(), display.height(), tr(fr, "origine", "origin"), display.left, display.top, tr(fr, "Dans OBS, superposer la capture de cet écran et la source Navigateur avec les mêmes dimensions et proportions. Liste actualisée automatiquement.", "In OBS, align this monitor capture and the Browser Source with matching size and proportions. Monitor list refreshes automatically.")));
        } else {
            hint.set_text(tr(fr, "Écran indisponible : cercle suspendu. Choisir un écran connecté ou rebrancher celui-ci. Clavier et boutons restent actifs.", "Monitor unavailable: ring suspended. Choose a connected monitor or reconnect this one. Keys and mouse buttons remain active."));
        }
        glib::ControlFlow::Continue
    };
    let _ = update();
    glib::timeout_add_local(Duration::from_secs(1), update);
}
