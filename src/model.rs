use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Ready,
    Key {
        label: String,
    },
    Mouse {
        button: u8,
        pressed: bool,
        x: Option<f64>,
        y: Option<f64>,
    },
    DeviceStatus {
        message: String,
    },
    Clear,
    Config {
        size: u32,
        duration: u32,
        dark: bool,
        #[serde(default)]
        halo: bool,
    },
}

pub fn modifier(code: u16) -> bool {
    matches!(code, 29 | 97 | 42 | 54 | 56 | 100 | 125 | 126)
}

pub fn shortcut_allowed(code: u16, ctrl: bool, alt: bool, super_key: bool, altgr: bool) -> bool {
    // AltGr is text entry, not a shortcut modifier. Never infer Ctrl+Alt from it.
    !modifier(code)
        && ((!altgr && (ctrl || alt || super_key))
            || matches!(code, 1 | 14 | 15 | 28 | 59..=68 | 87 | 88 | 96 | 102..=111))
}

pub fn named_key(code: u16) -> Option<&'static str> {
    Some(match code {
        1 => "Esc",
        14 => "Backspace",
        15 => "Tab",
        28 | 96 => "Enter",
        57 => "Space",
        59 => "F1",
        60 => "F2",
        61 => "F3",
        62 => "F4",
        63 => "F5",
        64 => "F6",
        65 => "F7",
        66 => "F8",
        67 => "F9",
        68 => "F10",
        87 => "F11",
        88 => "F12",
        102 => "Home",
        103 => "↑",
        104 => "PageUp",
        105 => "←",
        106 => "→",
        107 => "End",
        108 => "↓",
        109 => "PageDown",
        110 => "Insert",
        111 => "Delete",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_mode_does_not_emit_text_or_altgr() {
        for key in [16, 30, 46, 57] {
            assert!(!shortcut_allowed(key, false, false, false, false));
            assert!(!shortcut_allowed(key, true, true, false, true));
        }
        assert!(shortcut_allowed(46, true, false, false, false));
        assert!(shortcut_allowed(88, false, false, false, false));
        assert!(!shortcut_allowed(29, true, false, false, false));
    }
    #[test]
    fn protocol_does_not_need_raw_key_events() {
        let value = serde_json::to_value(Event::Key {
            label: "Ctrl+C".into(),
        })
        .unwrap();
        assert_eq!(value, serde_json::json!({"type":"key", "label":"Ctrl+C"}));
    }
}
