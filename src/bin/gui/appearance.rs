use super::tr;
use gtk4::{
    gdk, glib, prelude::*, Box as GtkBox, Button, ColorButton, ComboBoxText, DrawingArea, Expander,
    GestureDrag, Grid, Label, Orientation, SpinButton,
};
use keycast_bridge::{
    appearance::{preferences_path, Appearance},
    server::Bridge,
};
use std::{cell::Cell, rc::Rc, sync::Arc, time::Duration};

fn rgba(color: &str) -> gdk::RGBA {
    gdk::RGBA::parse(color).unwrap_or(gdk::RGBA::WHITE)
}
fn hex(color: gdk::RGBA) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (color.red() * 255.0).round() as u8,
        (color.green() * 255.0).round() as u8,
        (color.blue() * 255.0).round() as u8
    )
}
fn paint(context: &gtk4::cairo::Context, color: &str) {
    let c = rgba(color);
    context.set_source_rgb(c.red().into(), c.green().into(), c.blue().into());
}
fn colors(a: &Appearance) -> [&str; 6] {
    [
        &a.background,
        &a.key_background,
        &a.text,
        &a.accent,
        &a.right_click,
        &a.middle_click,
    ]
}
fn save(state: &Bridge, failed: &Cell<bool>) {
    failed.set(preferences_path().is_none_or(|path| state.appearance().save(&path).is_err()));
}

pub fn build(root: &GtkBox, state: Arc<Bridge>, language: ComboBoxText) {
    let expander = Expander::new(None);
    expander.set_expanded(true);
    let panel = GtkBox::new(Orientation::Vertical, 8);
    expander.set_child(Some(&panel));
    root.append(&expander);
    let help = Label::new(None);
    help.set_wrap(true);
    help.set_xalign(0.0);
    panel.append(&help);
    let preview = DrawingArea::new();
    preview.set_content_height(190);
    preview.set_hexpand(true);
    preview.set_cursor_from_name(Some("move"));
    panel.append(&preview);
    let position = GtkBox::new(Orientation::Horizontal, 8);
    let presets = ComboBoxText::new();
    presets.set_hexpand(true);
    position.append(&presets);
    let x_label = Label::new(Some("X (%)"));
    let y_label = Label::new(Some("Y (%)"));
    let x = SpinButton::with_range(0.0, 100.0, 1.0);
    let y = SpinButton::with_range(0.0, 100.0, 1.0);
    x.set_digits(1);
    y.set_digits(1);
    for (label, spin) in [(&x_label, &x), (&y_label, &y)] {
        position.append(label);
        position.append(spin);
    }
    panel.append(&position);
    let grid = Grid::new();
    grid.set_column_spacing(12);
    grid.set_row_spacing(6);
    let mut swatches = Vec::new();
    let mut labels = Vec::new();
    let updating = Rc::new(Cell::new(false));
    let failed = Rc::new(Cell::new(false));
    for i in 0..6 {
        let label = Label::new(None);
        label.set_xalign(0.0);
        let button = ColorButton::new();
        button.set_use_alpha(false);
        grid.attach(&label, (i % 2) * 2, i / 2, 1, 1);
        grid.attach(&button, (i % 2) * 2 + 1, i / 2, 1, 1);
        let s = state.clone();
        let failed = failed.clone();
        let preview = preview.clone();
        button.connect_color_set(move |button| {
            let mut a = s.appearance();
            let color = hex(button.rgba());
            match i {
                0 => a.background = color,
                1 => a.key_background = color,
                2 => a.text = color,
                3 => a.accent = color,
                4 => a.right_click = color,
                _ => a.middle_click = color,
            }
            s.set_appearance(a);
            save(&s, &failed);
            preview.queue_draw();
        });
        swatches.push(button);
        labels.push(label);
    }
    panel.append(&grid);
    let palette = GtkBox::new(Orientation::Horizontal, 8);
    let dark = Button::new();
    let light = Button::new();
    let reset = Button::new();
    for b in [&dark, &light, &reset] {
        palette.append(b);
    }
    panel.append(&palette);
    for (button, mode) in [(&dark, 0), (&light, 1), (&reset, 2)] {
        let s = state.clone();
        let failed = failed.clone();
        let preview = preview.clone();
        button.connect_clicked(move |_| {
            let previous = s.appearance();
            let mut a = Appearance {
                canvas_width: previous.canvas_width,
                canvas_height: previous.canvas_height,
                ..Default::default()
            };
            if mode != 2 {
                a.x = previous.x;
                a.y = previous.y;
            }
            if mode == 1 {
                a.light();
            }
            s.set_appearance(a);
            save(&s, &failed);
            preview.queue_draw();
        });
    }
    let saved = Label::new(None);
    saved.set_xalign(0.0);
    saved.set_wrap(true);
    panel.append(&saved);
    for (spin, horizontal) in [(&x, true), (&y, false)] {
        let s = state.clone();
        let updating = updating.clone();
        let failed = failed.clone();
        let preview = preview.clone();
        spin.connect_value_changed(move |spin| {
            if updating.get() {
                return;
            }
            let mut a = s.appearance();
            if horizontal {
                a.x = spin.value();
            } else {
                a.y = spin.value();
            }
            s.set_appearance(a);
            save(&s, &failed);
            preview.queue_draw();
        });
    }
    {
        let s = state.clone();
        let updating = updating.clone();
        let failed = failed.clone();
        let preview = preview.clone();
        presets.connect_changed(move |combo| {
            if updating.get() {
                return;
            }
            if let Some(index) = combo.active_id().and_then(|id| id.parse::<u32>().ok()) {
                let mut a = s.appearance();
                a.x = f64::from(index % 3) * 50.0;
                a.y = f64::from(index / 3) * 50.0;
                s.set_appearance(a);
                save(&s, &failed);
                preview.queue_draw();
            }
        });
    }
    {
        let s = state.clone();
        preview.set_draw_func(move |_, cr, width, height| {
            let (w, h) = (f64::from(width), f64::from(height));
            let a = s.appearance();
            cr.set_source_rgb(0.12, 0.14, 0.18);
            let _ = cr.paint();
            let (ox, oy, w, h) = canvas_rect(w, h, &a);
            cr.set_source_rgb(0.18, 0.21, 0.26);
            cr.rectangle(ox, oy, w, h);
            let _ = cr.fill();
            let _ = cr.save();
            cr.translate(ox, oy);
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.12);
            for i in 1..4 {
                cr.move_to(w * f64::from(i) / 4.0, 0.0);
                cr.line_to(w * f64::from(i) / 4.0, h);
                cr.move_to(0.0, h * f64::from(i) / 4.0);
                cr.line_to(w, h * f64::from(i) / 4.0);
            }
            let _ = cr.stroke();
            // Schematic sample: its travel, like the browser overlay, is the free space.
            let (bw, bh) = ((w * 0.38).min(210.0), 48.0);
            let (left, top) = (
                8.0 + (w - bw - 16.0).max(0.0) * a.x / 100.0,
                8.0 + (h - bh - 16.0).max(0.0) * a.y / 100.0,
            );
            paint(cr, &a.background);
            cr.rectangle(left, top, bw, bh);
            let _ = cr.fill();
            paint(cr, &a.key_background);
            cr.rectangle(left + 6.0, top + 6.0, bw - 12.0, bh - 12.0);
            let _ = cr.fill();
            paint(cr, &a.accent);
            cr.set_line_width(2.0);
            cr.rectangle(left + 6.0, top + 6.0, bw - 12.0, bh - 12.0);
            let _ = cr.stroke();
            paint(cr, &a.text);
            cr.set_font_size((bw / 10.0).min(17.0));
            cr.move_to(left + 14.0, top + 30.0);
            let _ = cr.show_text("Ctrl + Shift");
            let _ = cr.restore();
        });
    }
    let gesture = GestureDrag::new();
    gesture.set_button(1);
    let origin = Rc::new(Cell::new((50.0, 100.0)));
    {
        let s = state.clone();
        let origin = origin.clone();
        gesture.connect_drag_begin(move |_, _, _| {
            let a = s.appearance();
            origin.set((a.x, a.y));
        });
    }
    {
        let s = state.clone();
        let origin = origin.clone();
        let preview = preview.clone();
        gesture.connect_drag_update(move |_, dx, dy| {
            let w = f64::from(preview.width());
            let h = f64::from(preview.height());
            let mut a = s.appearance();
            let (_, _, w, h) = canvas_rect(w, h, &a);
            a.x = origin.get().0 + dx / (w - (w * 0.38).min(210.0) - 16.0).max(1.0) * 100.0;
            a.y = origin.get().1 + dy / (h - 64.0).max(1.0) * 100.0;
            s.set_appearance(a);
            preview.queue_draw();
        });
    }
    {
        let s = state.clone();
        let failed = failed.clone();
        gesture.connect_drag_end(move |_, _, _| save(&s, &failed));
    }
    preview.add_controller(gesture);
    let weak = root.downgrade();
    let mut previous_fr = None;
    let mut previous_appearance = None;
    let mut update = move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let fr = language.active_id().is_some_and(|id| id == "fr");
        let a = state.appearance();
        updating.set(true);
        if previous_fr != Some(fr) {
            expander.set_label(Some(tr(fr, "Position et couleurs", "Position and colors")));
            help.set_text(tr(fr, "Glissez le bloc dans l’aperçu schématique, ou réglez X/Y (0–100 %). Vérifiez le rendu exact avec « Tester le rendu » dans OBS. Les réglages s’appliquent en direct.", "Drag the block in the schematic preview, or adjust X/Y (0–100%). Check the exact result with “Test overlay” in OBS. Changes apply live."));
            preview.set_tooltip_text(Some(tr(
                fr,
                "Glisser pour déplacer ; les champs X et Y permettent aussi le réglage au clavier.",
                "Drag to move; X and Y fields also support keyboard adjustment.",
            )));
            presets.remove_all();
            for (i, (french, english)) in [
                ("Haut gauche", "Top left"),
                ("Haut centre", "Top center"),
                ("Haut droite", "Top right"),
                ("Centre gauche", "Middle left"),
                ("Centre", "Center"),
                ("Centre droite", "Middle right"),
                ("Bas gauche", "Bottom left"),
                ("Bas centre", "Bottom center"),
                ("Bas droite", "Bottom right"),
            ]
            .iter()
            .enumerate()
            {
                presets.append(Some(&i.to_string()), tr(fr, french, english));
            }
            presets.append(Some("custom"), tr(fr, "Position libre", "Custom position"));
            for ((label, button), (french, english)) in labels.iter().zip(&swatches).zip([
                ("Fond", "Background"),
                ("Touches", "Key background"),
                ("Texte", "Text"),
                ("Accent / clic gauche", "Accent / left click"),
                ("Clic droit", "Right click"),
                ("Clic molette", "Middle click"),
            ]) {
                let title = tr(fr, french, english);
                label.set_text(title);
                button.set_title(title);
                button.set_tooltip_text(Some(title));
            }
            dark.set_label(tr(fr, "Palette sombre", "Dark palette"));
            light.set_label(tr(fr, "Palette claire", "Light palette"));
            reset.set_label(tr(fr, "Tout réinitialiser", "Reset all"));
        }
        if previous_appearance.as_ref() != Some(&a) || previous_fr != Some(fr) {
            x.set_value(a.x);
            y.set_value(a.y);
            let preset = if [0.0, 50.0, 100.0].contains(&a.x) && [0.0, 50.0, 100.0].contains(&a.y) {
                ((a.y / 50.0) as u32 * 3 + (a.x / 50.0) as u32).to_string()
            } else {
                "custom".into()
            };
            presets.set_active_id(Some(&preset));
            for (button, color) in swatches.iter().zip(colors(&a)) {
                button.set_rgba(&rgba(color));
            }
            preview.queue_draw();
        }
        saved.set_text(if failed.get() { tr(fr, "Réglages appliqués, mais enregistrement impossible. Ils seront perdus à la fermeture.", "Settings applied, but could not be saved. They will be lost on exit.") } else { tr(fr, "Position et couleurs mémorisées automatiquement. Le cercle au clic reste lié au pointeur.", "Position and colors are saved automatically. The click ring stays at the pointer.") });
        updating.set(false);
        previous_fr = Some(fr);
        previous_appearance = Some(a);
        glib::ControlFlow::Continue
    };
    let _ = update();
    glib::timeout_add_local(Duration::from_millis(100), update);
}

pub fn canvas_controls(root: &GtkBox, state: Arc<Bridge>, language: ComboBoxText) {
    let heading = Label::new(None);
    heading.set_xalign(0.0);
    heading.add_css_class("heading");
    root.append(&heading);
    let presets = ComboBoxText::new();
    for (id, title) in [
        ("1920x1080", "1920 × 1080 — Full HD"),
        ("1280x720", "1280 × 720 — HD"),
        ("2560x1440", "2560 × 1440 — QHD"),
        ("3840x2160", "3840 × 2160 — UHD / 4K"),
        ("1080x1920", "1080 × 1920 — 9:16"),
        ("1080x1080", "1080 × 1080 — 1:1"),
    ] {
        presets.append(Some(id), title);
    }
    presets.append(Some("custom"), "Personnalisé / Custom");
    root.append(&presets);
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let width_label = Label::new(None);
    let height_label = Label::new(None);
    let width = SpinButton::with_range(160.0, 7680.0, 1.0);
    let height = SpinButton::with_range(160.0, 7680.0, 1.0);
    row.append(&width_label);
    row.append(&width);
    row.append(&height_label);
    row.append(&height);
    root.append(&row);
    let hint = Label::new(None);
    hint.set_wrap(true);
    hint.set_xalign(0.0);
    root.append(&hint);
    let saved = Label::new(None);
    saved.set_wrap(true);
    saved.set_xalign(0.0);
    root.append(&saved);
    let updating = Rc::new(Cell::new(false));
    let failed = Rc::new(Cell::new(false));
    for (spin, horizontal) in [(&width, true), (&height, false)] {
        let state = state.clone();
        let updating = updating.clone();
        let failed = failed.clone();
        spin.connect_value_changed(move |spin| {
            if updating.get() {
                return;
            }
            let mut a = state.appearance();
            if horizontal {
                a.canvas_width = spin.value_as_int() as u32;
            } else {
                a.canvas_height = spin.value_as_int() as u32;
            }
            state.set_appearance(a);
            save(&state, &failed);
        });
    }
    {
        let state = state.clone();
        let updating = updating.clone();
        let failed = failed.clone();
        presets.connect_changed(move |combo| {
            if updating.get() {
                return;
            }
            if let Some(id) = combo.active_id() {
                if let Some((w, h)) = id
                    .split_once('x')
                    .and_then(|(w, h)| Some((w.parse::<u32>().ok()?, h.parse::<u32>().ok()?)))
                {
                    let mut a = state.appearance();
                    a.canvas_width = w;
                    a.canvas_height = h;
                    state.set_appearance(a);
                    save(&state, &failed);
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
        let a = state.appearance();
        if previous != Some((a.canvas_width, a.canvas_height)) {
            updating.set(true);
            width.set_value(a.canvas_width.into());
            height.set_value(a.canvas_height.into());
            if !presets.set_active_id(Some(&format!("{}x{}", a.canvas_width, a.canvas_height))) {
                presets.set_active_id(Some("custom"));
            }
            updating.set(false);
            previous = Some((a.canvas_width, a.canvas_height));
        }
        heading.set_text(tr(fr, "Format de l’incrustation OBS", "OBS overlay canvas"));
        width_label.set_text(tr(fr, "Largeur (px)", "Width (px)"));
        height_label.set_text(tr(fr, "Hauteur (px)", "Height (px)"));
        hint.set_text(&format!("{} {} × {}. {}", tr(fr, "Dans les propriétés de la source Navigateur OBS, reporter", "In OBS Browser Source properties, enter"), a.canvas_width, a.canvas_height, tr(fr, "L’application ne modifie pas ces propriétés automatiquement. Si le format diffère, l’incrustation s’ajuste sans déformation avec des marges transparentes.", "The application does not change those properties automatically. If the format differs, the overlay fits without distortion with transparent margins.")));
        saved.set_text(if failed.get() {
            tr(
                fr,
                "Format appliqué, mais sauvegarde impossible.",
                "Canvas applied, but could not be saved.",
            )
        } else {
            tr(
                fr,
                "Format mémorisé pour le prochain lancement.",
                "Canvas saved for the next launch.",
            )
        });
        glib::ControlFlow::Continue
    };
    let _ = update();
    glib::timeout_add_local(Duration::from_millis(150), update);
}

fn canvas_rect(width: f64, height: f64, a: &Appearance) -> (f64, f64, f64, f64) {
    let scale = (width / f64::from(a.canvas_width)).min(height / f64::from(a.canvas_height));
    let w = f64::from(a.canvas_width) * scale;
    let h = f64::from(a.canvas_height) * scale;
    ((width - w) / 2.0, (height - h) / 2.0, w, h)
}
