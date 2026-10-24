use crate::{
    appearance::Appearance,
    presentation::{Mode, Scene, SpotlightShape, TrailStyle},
};
use gtk4::cairo::{Context, FillRule, LineCap, LineJoin, Operator};
use std::f64::consts::TAU;
fn color(c: &Context, hex: &str, alpha: f64) {
    let n = u32::from_str_radix(&hex[1..], 16).unwrap_or(0xa879ff);
    c.set_source_rgba(
        f64::from((n >> 16) & 255) / 255.0,
        f64::from((n >> 8) & 255) / 255.0,
        f64::from(n & 255) / 255.0,
        alpha,
    );
}
fn circle(c: &Context, x: f64, y: f64, r: f64) {
    c.new_sub_path();
    c.arc(x, y, r, 0.0, TAU);
}
fn ring(
    c: &Context,
    point: (f64, f64),
    radius: f64,
    width: f64,
    hex: &str,
    alpha: f64,
    neon: bool,
) {
    if neon {
        for n in (1..=3).rev() {
            color(c, hex, alpha * 0.09);
            c.set_line_width(width + f64::from(n) * 5.0);
            circle(c, point.0, point.1, radius);
            let _ = c.stroke();
        }
    }
    color(c, hex, alpha);
    c.set_line_width(width);
    circle(c, point.0, point.1, radius);
    let _ = c.stroke();
}
pub(super) fn draw(
    c: &Context,
    size: (f64, f64),
    scene: &Scene,
    a: &Appearance,
    mode: Mode,
    t: f64,
) {
    let (w, h) = size;
    let s = &a.presentation;
    let r = &a.ring;
    let (x, y) = (scene.pointer.x, scene.pointer.y);
    // The caller supplies a cleared surface, or a frozen desktop image in zoom mode.
    c.set_operator(Operator::Over);
    if mode == Mode::Whiteboard {
        c.set_source_rgb(1.0, 1.0, 1.0);
        let _ = c.paint();
    }
    if s.spotlight && mode == Mode::Normal {
        c.set_source_rgba(0.0, 0.0, 0.0, f64::from(s.dimming) / 100.0);
        c.rectangle(0.0, 0.0, w, h);
        let radius = f64::from(s.spotlight_radius);
        if s.spotlight_shape == SpotlightShape::Square {
            c.rectangle(x - radius, y - radius, 2.0 * radius, 2.0 * radius);
        } else {
            circle(c, x, y, radius);
        }
        c.set_fill_rule(FillRule::EvenOdd);
        let _ = c.fill();
        c.set_fill_rule(FillRule::Winding);
    }
    c.set_line_cap(LineCap::Round);
    c.set_line_join(LineJoin::Round);
    if s.trail && mode == Mode::Normal {
        match s.trail_style {
            TrailStyle::Dash => c.set_dash(&[12.0, 10.0], 0.0),
            TrailStyle::Dot => c.set_dash(&[0.1, 10.0], 0.0),
            _ => (),
        }
        let points: Vec<_> = scene.trail.iter().collect();
        let mut distance = 0.0;
        for (i, pair) in points.windows(2).enumerate() {
            let (prev, p) = (pair[0].point, pair[1].point);
            match s.trail_style {
                TrailStyle::Dash => c.set_dash(&[12.0, 10.0], distance),
                TrailStyle::Dot => c.set_dash(&[0.1, 10.0], distance),
                _ => (),
            }
            distance += prev.distance(p);
            let alpha = (1.0 - (t - pair[1].time) / 0.65).clamp(0.0, 1.0);
            color(c, &r.color, alpha * 0.85);
            c.set_line_width(f64::from(r.thickness));
            if matches!(s.trail_style, TrailStyle::Particle | TrailStyle::Star) {
                if i % 3 != 0 {
                    continue;
                }
                if s.trail_style == TrailStyle::Star {
                    for k in 0..10 {
                        let angle = f64::from(k) * TAU / 10.0 - TAU / 4.0;
                        let size = if k % 2 == 0 { 7.0 } else { 3.0 } * alpha;
                        let px = p.x + angle.cos() * size;
                        let py = p.y + angle.sin() * size;
                        if k == 0 {
                            c.move_to(px, py);
                        } else {
                            c.line_to(px, py);
                        }
                    }
                    c.close_path();
                    let _ = c.fill();
                } else {
                    circle(c, p.x, p.y, (4.0 * alpha).max(1.0));
                    let _ = c.fill();
                }
            } else {
                if s.trail_style == TrailStyle::Glow {
                    for n in (1..=3).rev() {
                        color(c, &r.color, alpha * 0.07);
                        c.set_line_width(f64::from(r.thickness + n * 4));
                        c.move_to(prev.x, prev.y);
                        c.line_to(p.x, p.y);
                        let _ = c.stroke();
                    }
                    color(c, &r.color, alpha);
                    c.set_line_width(f64::from(r.thickness));
                }
                c.move_to(prev.x, prev.y);
                c.line_to(p.x, p.y);
                let _ = c.stroke();
            }
        }
        c.set_dash(&[], 0.0);
    }
    if s.ring && mode == Mode::Normal {
        ring(
            c,
            (x, y),
            f64::from(r.radius),
            f64::from(r.thickness),
            &r.color,
            if r.pulse {
                0.75 + 0.25 * (t * 4.0).sin()
            } else {
                1.0
            },
            r.neon,
        );
        for click in &scene.clicks {
            let progress = ((t - click.time) / 0.5).clamp(0.0, 1.0);
            ring(
                c,
                (click.point.x, click.point.y),
                f64::from(r.radius) * (0.7 + progress * 0.7),
                f64::from(r.thickness),
                &r.color,
                1.0 - progress,
                r.neon,
            );
        }
    }
    if scene.located_until > t && mode == Mode::Normal {
        ring(
            c,
            (x, y),
            70.0 + (scene.located_until - t) * 90.0,
            5.0,
            &r.color,
            ((scene.located_until - t) * 3.0).min(1.0),
            true,
        );
    }
    for stroke in &scene.strokes {
        color(c, &stroke.color, 1.0);
        c.set_line_width(stroke.width);
        if stroke.points.len() == 1 {
            circle(
                c,
                stroke.points[0].x,
                stroke.points[0].y,
                stroke.width / 2.0,
            );
            let _ = c.fill();
        } else if let Some(first) = stroke.points.first() {
            c.move_to(first.x, first.y);
            for p in &stroke.points[1..] {
                c.line_to(p.x, p.y);
            }
            let _ = c.stroke();
        }
    }
    if mode != Mode::Normal {
        c.set_source_rgba(0.08, 0.09, 0.12, 0.85);
        c.rectangle(12.0, 12.0, 610.0_f64.min(w - 24.0), 36.0);
        let _ = c.fill();
        c.set_source_rgb(1.0, 1.0, 1.0);
        c.set_font_size(15.0);
        c.move_to(22.0, 35.0);
        let _ = c.show_text(if mode == Mode::Zoom {
            "Zoom · Scroll / + / − · Esc"
        } else {
            "Draw · Ctrl+Z: undo · Backspace: clear · Esc"
        });
    }
}
