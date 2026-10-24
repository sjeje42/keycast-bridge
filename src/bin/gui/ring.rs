use super::{
    appearance_controls::{hex, rgba, save},
    tr,
};
use gtk4::{
    glib, prelude::*, Box as GtkBox, Button, CheckButton, ColorButton, ComboBoxText, DrawingArea,
    Grid, Label, SpinButton,
};
use keycast_bridge::{appearance::RingAppearance, server::Bridge};
use std::{
    cell::Cell,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

pub fn build(root: &GtkBox, tab: &Label, state: Arc<Bridge>, language: ComboBoxText) {
    let help = Label::new(None);
    help.set_wrap(true);
    help.set_xalign(0.0);
    root.append(&help);
    let preview = DrawingArea::new();
    preview.set_content_height(200);
    preview.set_hexpand(true);
    root.append(&preview);
    let grid = Grid::new();
    grid.set_column_spacing(16);
    grid.set_row_spacing(10);
    let color_label = Label::new(None);
    let radius_label = Label::new(None);
    let thickness_label = Label::new(None);
    let color = ColorButton::new();
    color.set_use_alpha(false);
    let radius = SpinButton::with_range(10.0, 200.0, 1.0);
    let thickness = SpinButton::with_range(1.0, 20.0, 1.0);
    for (row, label) in [&color_label, &radius_label, &thickness_label]
        .iter()
        .enumerate()
    {
        label.set_xalign(0.0);
        grid.attach(*label, 0, row as i32, 1, 1);
    }
    grid.attach(&color, 1, 0, 1, 1);
    grid.attach(&radius, 1, 1, 1, 1);
    grid.attach(&thickness, 1, 2, 1, 1);
    root.append(&grid);
    let neon = CheckButton::new();
    let pulse = CheckButton::new();
    root.append(&neon);
    root.append(&pulse);
    let reset = Button::new();
    root.append(&reset);
    let saved = Label::new(None);
    saved.set_xalign(0.0);
    saved.set_wrap(true);
    root.append(&saved);
    let updating = Rc::new(Cell::new(false));
    let failed = Rc::new(Cell::new(false));
    {
        let s = state.clone();
        let failed = failed.clone();
        color.connect_color_set(move |button| {
            let mut a = s.appearance();
            a.ring.color = hex(button.rgba());
            s.set_appearance(a);
            save(&s, &failed);
        });
    }
    for (spin, is_radius) in [(&radius, true), (&thickness, false)] {
        let s = state.clone();
        let failed = failed.clone();
        let updating = updating.clone();
        spin.connect_value_changed(move |spin| {
            if updating.get() {
                return;
            }
            let mut a = s.appearance();
            if is_radius {
                a.ring.radius = spin.value_as_int() as u32;
            } else {
                a.ring.thickness = spin.value_as_int() as u32;
            }
            s.set_appearance(a);
            save(&s, &failed);
        });
    }
    for (check, is_neon) in [(&neon, true), (&pulse, false)] {
        let s = state.clone();
        let failed = failed.clone();
        let updating = updating.clone();
        check.connect_toggled(move |check| {
            if updating.get() {
                return;
            }
            let mut a = s.appearance();
            if is_neon {
                a.ring.neon = check.is_active();
            } else {
                a.ring.pulse = check.is_active();
            }
            s.set_appearance(a);
            save(&s, &failed);
        });
    }
    {
        let s = state.clone();
        let failed = failed.clone();
        reset.connect_clicked(move |_| {
            let mut a = s.appearance();
            a.ring = RingAppearance::default();
            s.set_appearance(a);
            save(&s, &failed);
        });
    }
    {
        let s = state.clone();
        let started = Instant::now();
        preview.set_draw_func(move |_, cr, width, height| {
            cr.set_source_rgb(0.12, 0.14, 0.18);
            let _ = cr.paint();
            let style = s.appearance().ring;
            // Reduced preview: the maximum pulsing ring and glow fit in the sample.
            let scale = (f64::from(width) / 620.0).min(f64::from(height) / 620.0);
            cr.translate(f64::from(width) / 2.0, f64::from(height) / 2.0);
            cr.scale(scale, scale);
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.6);
            cr.set_line_width(2.0);
            cr.move_to(-10.0, 0.0);
            cr.line_to(10.0, 0.0);
            cr.move_to(0.0, -10.0);
            cr.line_to(0.0, 10.0);
            let _ = cr.stroke();
            let elapsed = started.elapsed().as_secs_f64() % 1.2;
            if elapsed >= 0.5 {
                return;
            }
            let progress = elapsed / 0.5;
            let expansion = if style.pulse {
                0.7 + 0.7 * progress
            } else {
                1.0
            };
            cr.scale(expansion, expansion);
            let radius = f64::from(style.radius) - f64::from(style.thickness) / 2.0;
            let c = rgba(&style.color);
            if style.neon {
                for spread in [16.0, 10.0, 5.0] {
                    cr.set_source_rgba(
                        c.red().into(),
                        c.green().into(),
                        c.blue().into(),
                        0.12 * (1.0 - progress),
                    );
                    cr.set_line_width(f64::from(style.thickness) + spread);
                    cr.arc(0.0, 0.0, radius, 0.0, std::f64::consts::TAU);
                    let _ = cr.stroke();
                }
            }
            cr.set_source_rgba(
                c.red().into(),
                c.green().into(),
                c.blue().into(),
                1.0 - progress,
            );
            cr.set_line_width(f64::from(style.thickness));
            cr.arc(0.0, 0.0, radius, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
        });
        // GTK's frame clock only animates this preview while it is mapped.
        preview.add_tick_callback(|preview, _| {
            preview.queue_draw();
            glib::ControlFlow::Continue
        });
    }
    if std::env::var("KEYCAST_SMOKE_TEST").as_deref() == Ok("1") {
        let original = state.appearance();
        color.set_rgba(&rgba("#12ab34"));
        color.emit_by_name::<()>("color-set", &[]);
        radius.set_value(80.0);
        thickness.set_value(8.0);
        neon.set_active(true);
        pulse.set_active(true);
        pulse.set_active(false);
        let a = state.appearance();
        assert_eq!(
            a.ring,
            RingAppearance {
                color: "#12ab34".into(),
                radius: 80,
                thickness: 8,
                neon: true,
                pulse: false
            }
        );
        assert_eq!(a.accent, original.accent);
        reset.emit_clicked();
        assert_eq!(state.appearance().ring, RingAppearance::default());
        state.set_appearance(original);
        save(&state, &failed);
    }
    let weak = root.downgrade();
    let tab = tab.clone();
    let mut previous = None;
    let mut update = move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let fr = language.active_id().is_some_and(|id| id == "fr");
        let a = state.appearance().ring;
        if previous.as_ref() != Some(&a) {
            updating.set(true);
            color.set_rgba(&rgba(&a.color));
            radius.set_value(a.radius.into());
            thickness
                .adjustment()
                .set_upper(f64::from(a.radius.min(20)));
            thickness.set_value(a.thickness.into());
            neon.set_active(a.neon);
            pulse.set_active(a.pulse);
            updating.set(false);
            previous = Some(a);
        }
        tab.set_text(tr(fr, "Anneau au clic", "Click ring"));
        help.set_text(tr(fr,
            "Aperçu réduit et répété. Le rayon et l’épaisseur sont exprimés en pixels du canevas OBS, indépendamment de la taille des touches. Le néon ajoute une lueur ; la pulsation agrandit l’anneau (70–140 % du rayon) avant sa disparition après 0,5 s. Activez « Cercle au clic » dans la fenêtre principale.",
            "Reduced, repeating preview. Radius and thickness use OBS canvas pixels, independently of key size. Neon adds a glow; pulse expands the ring (70–140% of the radius) before it disappears after 0.5 s. Enable “Click ring” in the main window."));
        color_label.set_text(tr(fr, "Couleur", "Color"));
        color.set_title(tr(fr, "Couleur de l’anneau", "Ring color"));
        color.set_tooltip_text(Some(tr(fr, "Couleur de l’anneau", "Ring color")));
        radius_label.set_text(tr(fr, "Rayon de base (px)", "Base radius (px)"));
        thickness_label.set_text(tr(fr, "Épaisseur (px)", "Thickness (px)"));
        neon.set_label(Some(tr(fr, "Effet néon", "Neon glow")));
        pulse.set_label(Some(tr(fr, "Effet pulsation", "Pulse effect")));
        reset.set_label(tr(fr, "Réinitialiser l’anneau", "Reset ring"));
        saved.set_text(if failed.get() {
            tr(fr, "Réglages appliqués, mais enregistrement impossible. Ils seront perdus à la fermeture.", "Settings applied, but could not be saved. They will be lost on exit.")
        } else {
            tr(fr, "Réglages de l’anneau mémorisés automatiquement et appliqués en direct.", "Ring settings are saved automatically and applied live.")
        });
        glib::ControlFlow::Continue
    };
    let _ = update();
    glib::timeout_add_local(Duration::from_millis(100), update);
}
