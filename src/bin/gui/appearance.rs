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
            let mut a = Appearance::default();
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
