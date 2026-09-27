use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub x: f64,
    pub y: f64,
    pub background: String,
    pub key_background: String,
    pub text: String,
    pub accent: String,
    pub right_click: String,
    pub middle_click: String,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            canvas_width: 1920,
            canvas_height: 1080,
            x: 50.0,
            y: 100.0,
            background: "#10151f".into(),
            key_background: "#273142".into(),
            text: "#f7f9ff".into(),
            accent: "#a879ff".into(),
            right_click: "#ff9e64".into(),
            middle_click: "#74dfba".into(),
        }
    }
}

impl Appearance {
    pub fn sanitized(mut self) -> Self {
        let defaults = Self::default();
        self.canvas_width = self.canvas_width.clamp(160, 7680);
        self.canvas_height = self.canvas_height.clamp(160, 7680);
        self.x = finite_percent(self.x, defaults.x);
        self.y = finite_percent(self.y, defaults.y);
        for (value, fallback) in [
            (&mut self.background, defaults.background),
            (&mut self.key_background, defaults.key_background),
            (&mut self.text, defaults.text),
            (&mut self.accent, defaults.accent),
            (&mut self.right_click, defaults.right_click),
            (&mut self.middle_click, defaults.middle_click),
        ] {
            if value.len() != 7
                || !value.starts_with('#')
                || !value.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
            {
                *value = fallback;
            } else {
                value.make_ascii_lowercase();
            }
        }
        self
    }

    pub fn light(&mut self) {
        self.background = "#f4f7ff".into();
        self.key_background = "#ffffff".into();
        self.text = "#172033".into();
    }

    pub fn load(path: &Path) -> Option<Self> {
        // This file contains appearance preferences only, never captured input.
        let file = std::fs::File::open(path).ok()?;
        if file.metadata().ok()?.len() > 16_384 {
            return None;
        }
        serde_json::from_reader::<_, Self>(file)
            .ok()
            .map(Self::sanitized)
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let temporary = path.with_extension("json.tmp");
        std::fs::write(
            &temporary,
            serde_json::to_vec_pretty(&self.clone().sanitized())?,
        )?;
        std::fs::rename(temporary, path)
    }
}

pub fn finite_percent(value: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value.clamp(0.0, 100.0)
    } else {
        fallback
    }
}

pub fn preferences_path() -> Option<PathBuf> {
    #[cfg(windows)]
    let base = std::env::var_os("APPDATA").map(PathBuf::from)?;
    #[cfg(not(windows))]
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")))?;
    Some(base.join("keycast-bridge").join("appearance.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_positions_and_colors_cannot_escape_overlay() {
        let a = Appearance {
            x: f64::NAN,
            y: 999.0,
            accent: "url(https://example.test)".into(),
            text: "#ABCDEF".into(),
            ..Default::default()
        }
        .sanitized();
        assert_eq!((a.x, a.y), (50.0, 100.0));
        assert_eq!(a.accent, "#a879ff");
        assert_eq!(a.text, "#abcdef");
    }
    #[test]
    fn old_preferences_gain_default_canvas_and_sizes_are_bounded() {
        let old: Appearance = serde_json::from_str(r##"{"x":20,"accent":"#123456"}"##).unwrap();
        assert_eq!((old.canvas_width, old.canvas_height), (1920, 1080));
        let custom = Appearance {
            canvas_width: 0,
            canvas_height: 99999,
            ..old
        }
        .sanitized();
        assert_eq!((custom.canvas_width, custom.canvas_height), (160, 7680));
        assert_eq!(custom.x, 20.0);
        assert_eq!(custom.accent, "#123456");
    }
    #[test]
    fn preferences_round_trip_and_invalid_file_fallback() {
        let dir = std::env::temp_dir().join(format!("keycast-style-{}", uuid::Uuid::new_v4()));
        let path = dir.join("appearance.json");
        let a = Appearance {
            canvas_width: 1080,
            canvas_height: 1920,
            x: 12.5,
            y: 31.0,
            accent: "#12ab34".into(),
            ..Default::default()
        };
        a.save(&path).unwrap();
        assert_eq!(Appearance::load(&path), Some(a.clone()));
        let b = Appearance { y: 80.0, ..a };
        b.save(&path).unwrap();
        assert_eq!(Appearance::load(&path), Some(b));
        std::fs::write(&path, "invalid").unwrap();
        assert!(Appearance::load(&path).is_none());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
