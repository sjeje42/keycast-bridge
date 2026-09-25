use crate::model::{modifier, named_key, shortcut_allowed};
use std::collections::HashSet;
use xkbcommon::xkb;

pub struct Normalizer {
    map: xkb::Keymap,
    state: xkb::State,
    down: HashSet<u16>,
    all_keys: bool,
}

pub enum Action {
    Ignore,
    Stop,
    Label(String),
}

impl Normalizer {
    pub fn new(layout: &str, all_keys: bool) -> anyhow::Result<Self> {
        anyhow::ensure!(
            ["fr", "us", "gb", "de"].contains(&layout),
            "Unsupported layout"
        );
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let map = xkb::Keymap::new_from_names(
            &context,
            "evdev",
            "pc105",
            layout,
            "",
            None,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )
        .ok_or_else(|| anyhow::anyhow!("Cannot load XKB layout"))?;
        let state = xkb::State::new(&map);
        Ok(Self {
            map,
            state,
            down: HashSet::new(),
            all_keys,
        })
    }

    pub fn event(&mut self, code: u16, value: i32) -> Action {
        let key = xkb::Keycode::new(u32::from(code) + 8);
        if value == 0 {
            self.down.remove(&code);
            self.state.update_key(key, xkb::KeyDirection::Up);
            return Action::Ignore;
        }
        if value != 1 || !self.down.insert(code) {
            return Action::Ignore;
        }
        self.state.update_key(key, xkb::KeyDirection::Down);
        let ctrl = self.down.contains(&29) || self.down.contains(&97);
        let alt = self.down.contains(&56);
        let altgr = self.down.contains(&100);
        let shift = self.down.contains(&42) || self.down.contains(&54);
        let super_key = self.down.contains(&125) || self.down.contains(&126);
        if code == 88 && ctrl && alt {
            return Action::Stop;
        }
        if modifier(code)
            || (!self.all_keys && !shortcut_allowed(code, ctrl, alt, super_key, altgr))
        {
            return Action::Ignore;
        }
        // Resolve shortcut letters at base level so Ctrl never becomes a control character.
        let label = if let Some(name) = named_key(code) {
            name.to_owned()
        } else if ctrl || alt || super_key {
            self.map
                .key_get_syms_by_level(key, 0, 0)
                .first()
                .map(|sym| xkb::keysym_to_utf8(*sym).to_uppercase())
                .unwrap_or_default()
        } else {
            self.state.key_get_utf8(key)
        };
        if label.is_empty() || label.chars().any(char::is_control) {
            return Action::Ignore;
        }
        let mut parts = Vec::new();
        if ctrl {
            parts.push("Ctrl".to_owned());
        }
        if alt {
            parts.push("Alt".to_owned());
        }
        if super_key {
            parts.push("Super".to_owned());
        }
        if shift {
            parts.push("Shift".to_owned());
        }
        if altgr {
            parts.push("AltGr".to_owned());
        }
        parts.push(label);
        Action::Label(parts.join(" + "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn chord(layout: &str) -> String {
        let mut n = Normalizer::new(layout, false).unwrap();
        n.event(29, 1);
        match n.event(16, 1) {
            Action::Label(s) => s,
            _ => panic!("missing chord"),
        }
    }
    #[test]
    fn azerty_and_qwerty_follow_layout() {
        assert_eq!(chord("fr"), "Ctrl + A");
        assert_eq!(chord("us"), "Ctrl + Q");
    }
    #[test]
    fn pause_is_never_broadcast() {
        let mut n = Normalizer::new("fr", true).unwrap();
        n.event(29, 1);
        n.event(56, 1);
        assert!(matches!(n.event(88, 1), Action::Stop));
    }
    #[test]
    fn repeats_and_released_modifiers_do_not_leak() {
        let mut n = Normalizer::new("us", false).unwrap();
        n.event(29, 1);
        assert!(matches!(n.event(46, 1), Action::Label(_)));
        assert!(matches!(n.event(46, 2), Action::Ignore));
        n.event(46, 0);
        n.event(29, 0);
        assert!(matches!(n.event(46, 1), Action::Ignore));
    }
}
