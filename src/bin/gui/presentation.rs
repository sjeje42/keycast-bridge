use super::{appearance_controls, tr};
use gtk4::{
    glib, prelude::*, Box as GtkBox, Button, CheckButton, ColorButton, ComboBoxText, Label,
    Orientation, Scale, SpinButton,
};
use keycast_bridge::{
    desktop::Controller,
    presentation::{Mode, SpotlightShape, TrailStyle},
    server::Bridge,
};
use std::{cell::Cell, rc::Rc, sync::Arc, time::Duration};

pub(super) fn build(
    panel: &GtkBox,
    tab: &Label,
    state: Arc<Bridge>,
    language: ComboBoxText,
    controller: Arc<Controller>,
) {
    let active = CheckButton::new();
    panel.append(&active);
    let status = Label::new(None);
    status.set_wrap(true);
    status.set_xalign(0.0);
    panel.append(&status);
    let actions = GtkBox::new(Orientation::Horizontal, 8);
    let draw = Button::new();
    let board = Button::new();
    let zoom = Button::new();
    let exit = Button::new();
    for b in [&draw, &board, &zoom, &exit] {
        actions.append(b);
    }
    panel.append(&actions);
    let hint = Label::new(None);
    hint.set_wrap(true);
    hint.set_xalign(0.0);
    panel.append(&hint);
    let ring = CheckButton::new();
    let trail = CheckButton::new();
    let shake = CheckButton::new();
    let spotlight = CheckButton::new();
    let style = ComboBoxText::new();
    let shape = ComboBoxText::new();
    let radius = SpinButton::with_range(40.0, 600.0, 10.0);
    let dim = Scale::with_range(Orientation::Horizontal, 0.0, 90.0, 1.0);
    dim.set_draw_value(true);
    dim.set_hexpand(true);
    let pen = ColorButton::new();
    pen.set_use_alpha(false);
    let width = SpinButton::with_range(1.0, 40.0, 1.0);
    let factor = SpinButton::with_range(1.25, 8.0, 0.25);
    factor.set_digits(2);
    panel.append(&ring);
    panel.append(&trail);
    panel.append(&style);
    panel.append(&shake);
    panel.append(&spotlight);
    panel.append(&shape);
    let mut labels = Vec::new();
    for widget in [
        radius.clone().upcast::<gtk4::Widget>(),
        dim.clone().upcast(),
        pen.clone().upcast(),
        width.clone().upcast(),
        factor.clone().upcast(),
    ] {
        let row = GtkBox::new(Orientation::Horizontal, 8);
        let label = Label::new(None);
        label.set_xalign(0.0);
        label.set_hexpand(true);
        row.append(&label);
        row.append(&widget);
        panel.append(&row);
        labels.push(label);
    }
    let saved = Label::new(None);
    saved.set_wrap(true);
    saved.set_xalign(0.0);
    panel.append(&saved);
    let updating = Rc::new(Cell::new(false));
    let failed = Rc::new(Cell::new(false));
    {
        let c = controller.clone();
        let u = updating.clone();
        active.connect_toggled(move |b| {
            if !u.get() {
                c.enable(b.is_active());
            }
        });
    }
    for (button, mode) in [
        (&draw, Mode::Annotate),
        (&board, Mode::Whiteboard),
        (&zoom, Mode::Zoom),
        (&exit, Mode::Normal),
    ] {
        let c = controller.clone();
        button.connect_clicked(move |_| c.mode(mode));
    }
    for (button, index) in [(&ring, 0), (&trail, 1), (&shake, 2), (&spotlight, 3)] {
        let s = state.clone();
        let u = updating.clone();
        let f = failed.clone();
        button.connect_toggled(move |b| {
            if u.get() {
                return;
            }
            let mut a = s.appearance();
            let p = &mut a.presentation;
            match index {
                0 => p.ring = b.is_active(),
                1 => p.trail = b.is_active(),
                2 => p.shake = b.is_active(),
                _ => p.spotlight = b.is_active(),
            };
            s.set_appearance(a);
            appearance_controls::save(&s, &f);
        });
    }
    for (combo, is_style) in [(&style, true), (&shape, false)] {
        let s = state.clone();
        let u = updating.clone();
        let f = failed.clone();
        combo.connect_changed(move |b| {
            if u.get() {
                return;
            }
            let Some(i) = b.active() else {
                return;
            };
            let mut a = s.appearance();
            if is_style {
                a.presentation.trail_style = [
                    TrailStyle::Solid,
                    TrailStyle::Dash,
                    TrailStyle::Dot,
                    TrailStyle::Glow,
                    TrailStyle::Particle,
                    TrailStyle::Star,
                ][i as usize];
            } else {
                a.presentation.spotlight_shape = if i == 0 {
                    SpotlightShape::Circle
                } else {
                    SpotlightShape::Square
                };
            }
            s.set_appearance(a);
            appearance_controls::save(&s, &f);
        });
    }
    for (spin, index) in [(&radius, 0), (&width, 1), (&factor, 2)] {
        let s = state.clone();
        let u = updating.clone();
        let f = failed.clone();
        spin.connect_value_changed(move |b| {
            if u.get() {
                return;
            }
            let mut a = s.appearance();
            match index {
                0 => a.presentation.spotlight_radius = b.value_as_int() as u32,
                1 => a.presentation.pen_width = b.value_as_int() as u32,
                _ => a.presentation.zoom = b.value(),
            };
            s.set_appearance(a);
            appearance_controls::save(&s, &f);
        });
    }
    {
        let s = state.clone();
        let u = updating.clone();
        let f = failed.clone();
        dim.connect_value_changed(move |b| {
            if u.get() {
                return;
            }
            let mut a = s.appearance();
            a.presentation.dimming = b.value() as u32;
            s.set_appearance(a);
            appearance_controls::save(&s, &f);
        });
    }
    {
        let s = state.clone();
        let u = updating.clone();
        let f = failed.clone();
        pen.connect_color_set(move |b| {
            if u.get() {
                return;
            }
            let mut a = s.appearance();
            a.presentation.pen_color = appearance_controls::hex(b.rgba());
            s.set_appearance(a);
            appearance_controls::save(&s, &f);
        });
    }
    let weak = panel.downgrade();
    let tab = tab.clone();
    let mut previous_language = None;
    glib::timeout_add_local(Duration::from_millis(150), move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let fr = language.active_id().is_some_and(|id| id == "fr");
        let a = state.appearance();
        let p = &a.presentation;
        updating.set(true);
        tab.set_text(tr(fr, "Présentation", "Presentation"));
        active.set_label(Some(tr(
            fr,
            "Activer les outils sur le bureau",
            "Enable desktop presentation tools",
        )));
        active.set_active(controller.enabled());
        actions.set_sensitive(controller.enabled());
        let message = controller.status();
        status.set_text(match message.as_str() {
            "Normal" | "normal" => tr(fr, "Effets du bureau actifs", "Desktop effects active"),
            "Annotate" | "annotate" => tr(
                fr,
                "Annotation active — Échap pour quitter",
                "Annotation active — Esc to exit",
            ),
            "Whiteboard" | "whiteboard" => tr(
                fr,
                "Tableau blanc actif — Échap pour quitter",
                "Whiteboard active — Esc to exit",
            ),
            "Zoom" | "zoom" => tr(
                fr,
                "Zoom figé actif — Échap pour quitter",
                "Frozen zoom active — Esc to exit",
            ),
            "stopped" => tr(fr, "Outils arrêtés", "Tools stopped"),
            _ => &message,
        });
        draw.set_label(tr(fr, "Dessiner", "Draw"));
        board.set_label(tr(fr, "Tableau blanc", "Whiteboard"));
        zoom.set_label("Zoom");
        exit.set_label(tr(fr, "Quitter le mode", "Exit mode"));
        hint.set_text(tr(fr,"Sur l’écran du pointeur : Ctrl+Alt+D dessiner, Ctrl+Alt+B tableau blanc, Ctrl+Alt+Z zoom figé. Échap quitte le mode ; Ctrl+Alt+F12 arrête tout. GNOME 48 : activer l’extension Keycast Bridge après installation. Dans OBS, utiliser la capture d’écran pour voir ces effets.","On the pointer’s monitor: Ctrl+Alt+D draw, Ctrl+Alt+B whiteboard, Ctrl+Alt+Z frozen zoom. Esc exits the mode; Ctrl+Alt+F12 stops all tools. GNOME 48: enable the Keycast Bridge extension after installation. Use Display Capture in OBS to include these effects."));
        ring.set_label(Some(tr(
            fr,
            "Anneau autour du curseur (réglages dans Anneau)",
            "Pointer ring (settings in Ring)",
        )));
        trail.set_label(Some(tr(fr, "Traînée de souris", "Mouse trail")));
        shake.set_label(Some(tr(fr, "Secouer pour localiser", "Shake to locate")));
        spotlight.set_label(Some(tr(fr, "Spotlight", "Spotlight")));
        for (b, v) in [
            (&ring, p.ring),
            (&trail, p.trail),
            (&shake, p.shake),
            (&spotlight, p.spotlight),
        ] {
            b.set_active(v);
        }
        if previous_language != Some(fr) {
            style.remove_all();
            for text in if fr {
                ["Plein", "Tiret", "Point", "Lueur", "Particule", "Étoile"]
            } else {
                ["Solid", "Dash", "Dot", "Glow", "Particle", "Star"]
            } {
                style.append_text(text);
            }
            shape.remove_all();
            shape.append_text(tr(fr, "Zone éclairée : cercle", "Illuminated area: circle"));
            shape.append_text(tr(fr, "Zone éclairée : carré", "Illuminated area: square"));
            previous_language = Some(fr);
        }
        style.set_active(Some(match p.trail_style {
            TrailStyle::Solid => 0,
            TrailStyle::Dash => 1,
            TrailStyle::Dot => 2,
            TrailStyle::Glow => 3,
            TrailStyle::Particle => 4,
            TrailStyle::Star => 5,
        }));
        shape.set_active(Some(if p.spotlight_shape == SpotlightShape::Circle {
            0
        } else {
            1
        }));
        radius.set_value(p.spotlight_radius.into());
        dim.set_value(p.dimming.into());
        pen.set_rgba(&appearance_controls::rgba(&p.pen_color));
        width.set_value(p.pen_width.into());
        factor.set_value(p.zoom);
        for (label, text) in labels.iter().zip(if fr {
            [
                "Rayon du spotlight (px)",
                "Assombrissement (%)",
                "Couleur du dessin",
                "Épaisseur du trait (px)",
                "Grossissement du zoom",
            ]
        } else {
            [
                "Spotlight radius (px)",
                "Dimming (%)",
                "Drawing color",
                "Stroke width (px)",
                "Zoom factor",
            ]
        }) {
            label.set_text(text);
        }
        saved.set_text(if failed.get(){tr(fr,"Échec de l’enregistrement des préférences.","Could not save preferences.")}else{tr(fr,"Styles mémorisés. Les outils restent arrêtés au lancement ; aucun dessin ni image du bureau n’est enregistré.","Styles are saved. Tools stay off at launch; drawings and desktop images are never saved.")});
        updating.set(false);
        glib::ControlFlow::Continue
    });
}
