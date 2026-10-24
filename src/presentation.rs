use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrailStyle {
    #[default]
    Solid,
    Dash,
    Dot,
    Glow,
    Particle,
    Star,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotlightShape {
    #[default]
    Circle,
    Square,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Normal,
    Annotate,
    Whiteboard,
    Zoom,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PresentationSettings {
    pub ring: bool,
    pub trail: bool,
    pub trail_style: TrailStyle,
    pub shake: bool,
    pub spotlight: bool,
    pub spotlight_shape: SpotlightShape,
    pub spotlight_radius: u32,
    pub dimming: u32,
    pub pen_color: String,
    pub pen_width: u32,
    pub zoom: f64,
}
impl Default for PresentationSettings {
    fn default() -> Self {
        Self {
            ring: true,
            trail: false,
            trail_style: TrailStyle::Solid,
            shake: true,
            spotlight: false,
            spotlight_shape: SpotlightShape::Circle,
            spotlight_radius: 150,
            dimming: 65,
            pen_color: "#ff4444".into(),
            pen_width: 4,
            zoom: 2.0,
        }
    }
}
impl PresentationSettings {
    pub fn sanitized(mut self) -> Self {
        self.spotlight_radius = self.spotlight_radius.clamp(40, 600);
        self.dimming = self.dimming.min(90);
        self.pen_width = self.pen_width.clamp(1, 40);
        self.zoom = if self.zoom.is_finite() {
            self.zoom.clamp(1.25, 8.0)
        } else {
            2.0
        };
        crate::appearance::sanitize_color(&mut self.pen_color, Self::default().pen_color);
        self
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn distance(self, other: Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub point: Point,
    pub time: f64,
}
#[derive(Clone, Debug)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: String,
    pub width: f64,
}
#[derive(Default)]
pub struct Scene {
    pub pointer: Point,
    pub trail: VecDeque<Sample>,
    pub clicks: VecDeque<Sample>,
    pub strokes: Vec<Stroke>,
    pub drawing: bool,
    pub located_until: f64,
    shake: VecDeque<Sample>,
    cooldown_until: f64,
}
impl Scene {
    pub fn move_to(&mut self, point: Point, time: f64, locate: bool) {
        if point.distance(self.pointer) >= 2.0 {
            self.trail.push_back(Sample { point, time });
            self.shake.push_back(Sample { point, time });
            if self.drawing {
                if let Some(stroke) = self.strokes.last_mut() {
                    if stroke.points.len() >= 2048 {
                        stroke.points = stroke.points.iter().step_by(2).copied().collect();
                    }
                    stroke.points.push(point);
                }
            }
        }
        self.pointer = point;
        self.expire(time);
        if locate && time >= self.cooldown_until && self.shake.len() >= 5 {
            let points: Vec<_> = self.shake.iter().map(|s| s.point).collect();
            let distance: f64 = points.windows(2).map(|w| w[0].distance(w[1])).sum();
            let reversals = points
                .windows(3)
                .filter(|w| {
                    let a = (w[1].x - w[0].x, w[1].y - w[0].y);
                    let b = (w[2].x - w[1].x, w[2].y - w[1].y);
                    a.0 * b.0 + a.1 * b.1 < -100.0
                })
                .count();
            if distance > 500.0 && reversals >= 3 {
                self.located_until = time + 0.9;
                self.cooldown_until = time + 1.5;
                self.shake.clear();
            }
        }
    }
    pub fn expire(&mut self, time: f64) {
        while self.trail.front().is_some_and(|s| time - s.time > 0.65) {
            self.trail.pop_front();
        }
        while self.trail.len() > 128 {
            self.trail.pop_front();
        }
        while self.shake.front().is_some_and(|s| time - s.time > 0.6) {
            self.shake.pop_front();
        }
        while self.shake.len() > 64 {
            self.shake.pop_front();
        }
        while self.clicks.front().is_some_and(|s| time - s.time > 0.5) {
            self.clicks.pop_front();
        }
    }
    pub fn click(&mut self, time: f64) {
        self.clicks.push_back(Sample {
            point: self.pointer,
            time,
        });
        while self.clicks.len() > 8 {
            self.clicks.pop_front();
        }
    }
    pub fn begin_stroke(&mut self, settings: &PresentationSettings) {
        if self.strokes.len() >= 128 {
            self.strokes.remove(0);
        }
        self.strokes.push(Stroke {
            points: vec![self.pointer],
            color: settings.pen_color.clone(),
            width: settings.pen_width.into(),
        });
        self.drawing = true;
    }
    pub fn undo(&mut self) {
        self.drawing = false;
        self.strokes.pop();
    }
    pub fn erase(&mut self) {
        self.drawing = false;
        self.strokes.clear();
    }
}
/// Top-left source position for a frozen, full-monitor zoom. Never samples outside the image.
pub fn zoom_origin(point: Point, width: f64, height: f64, zoom: f64) -> Point {
    Point {
        x: (point.x - width / zoom / 2.0).clamp(0.0, (width - width / zoom).max(0.0)),
        y: (point.y - height / zoom / 2.0).clamp(0.0, (height - height / zoom).max(0.0)),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shake_requires_reversals_not_fast_motion_or_jitter() {
        let mut straight = Scene::default();
        for i in 0..10 {
            straight.move_to(
                Point {
                    x: f64::from(i) * 100.0,
                    y: 0.0,
                },
                f64::from(i) * 0.04,
                true,
            );
        }
        assert_eq!(straight.located_until, 0.0);
        let mut shake = Scene::default();
        for i in 0..8 {
            shake.move_to(
                Point {
                    x: if i % 2 == 0 { 100.0 } else { 300.0 },
                    y: 0.0,
                },
                f64::from(i) * 0.06,
                true,
            );
        }
        assert!(shake.located_until > 0.8);
        let until = shake.located_until;
        shake.move_to(Point { x: 100.0, y: 0.0 }, 0.5, true);
        assert_eq!(shake.located_until, until);
        let mut jitter = Scene::default();
        for i in 0..20 {
            jitter.move_to(
                Point {
                    x: f64::from(i % 2) * 3.0,
                    y: 0.0,
                },
                f64::from(i) * 0.02,
                true,
            );
        }
        assert_eq!(jitter.located_until, 0.0);
    }
    #[test]
    fn drawing_is_bounded_and_undo_removes_one_stroke() {
        let mut s = Scene::default();
        let settings = PresentationSettings::default();
        s.begin_stroke(&settings);
        for i in 0..10000 {
            s.move_to(
                Point {
                    x: f64::from(i) * 3.0,
                    y: 0.0,
                },
                f64::from(i) * 0.01,
                false,
            );
        }
        assert!(s.strokes[0].points.len() <= 2048);
        s.drawing = false;
        s.begin_stroke(&settings);
        s.undo();
        assert_eq!(s.strokes.len(), 1);
        s.erase();
        assert!(s.strokes.is_empty());
        s.click(101.0);
        s.expire(102.0);
        assert!(s.trail.is_empty() && s.clicks.is_empty());
    }
    #[test]
    fn preferences_and_zoom_bounds() {
        let p = PresentationSettings {
            spotlight_radius: 0,
            dimming: 999,
            pen_width: 0,
            zoom: f64::NAN,
            ..Default::default()
        }
        .sanitized();
        assert_eq!((p.spotlight_radius, p.dimming, p.pen_width), (40, 90, 1));
        assert_eq!(p.zoom, 2.0);
        assert_eq!(
            zoom_origin(Point { x: 0.0, y: 0.0 }, 1920.0, 1080.0, 2.0),
            Point { x: 0.0, y: 0.0 }
        );
        assert_eq!(
            zoom_origin(
                Point {
                    x: 1920.0,
                    y: 1080.0
                },
                1920.0,
                1080.0,
                2.0
            ),
            Point { x: 960.0, y: 540.0 }
        );
    }
}
