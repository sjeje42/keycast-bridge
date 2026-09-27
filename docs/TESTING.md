# Validation / Acceptance tests

## Automated

```sh
npm --prefix web ci
npm --prefix web run check
npm --prefix web run build
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

Unit tests cover layout-dependent Ctrl+A/Ctrl+Q, modifier release, ignored repeat events, emergency stop suppression, shortcuts-only filtering, protocol shape, rejected origins/hosts/tokens and clearing on stop. They do not simulate real evdev permissions or Polkit.

The included `scripts/smoke.mjs` exercises a running demo over HTTP and WebSocket (Node.js 22). Build `keycast-bridge-demo` first and ensure port 48732 is free.

See [VALIDATION.md](VALIDATION.md) for checks actually completed for this alpha.

## Real Debian 13 + GNOME + OBS test checklist

- [ ] Build, install and launch from GNOME without sudo.
- [ ] UI starts stopped; FR/EN labels and keyboard focus work.
- [ ] OBS includes Browser Source; transparent preview and synthetic demo appear.
- [ ] Wrong token and foreign Origin cannot subscribe; only 127.0.0.1 listens.
- [ ] Cancel Polkit: stopped/error state, no capture. Start again successfully.
- [ ] Choose a non-keyboard: rejected. Select a real keyboard: capture active.
- [ ] FR Ctrl+A, Ctrl+C, Shift+Ctrl+V, Alt+Tab, Super+arrows, F1–F12.
- [ ] Switch both desktop and app to US: same physical Q position now yields Ctrl+Q.
- [ ] Default mode does not show unmodified letters or AltGr text.
- [ ] All-keys mode intentionally shows letters; disable after testing.
- [ ] Ctrl+Alt+F12 clears OBS and stops; no emergency chord is displayed.
- [ ] Stop during the authorization dialog, then authorize: no event reaches OBS.
- [ ] Stop/restart rapidly: old capture cannot publish into new session.
- [ ] Close app: capture ends, OBS clears. No helper remains. Disconnecting a keyboard clears its state while capture continues.
- [ ] Restart app: old URL fails, new URL works; preferences intentionally reset.
- [ ] OBS reconnect / hidden source / slow consumer: no stale key replay.
- [ ] Long session: no persistent key logs or increasing memory backlog.

**Not guaranteed in this alpha:** initial Caps/Num Lock state, automatic desktop layout tracking, multi-keyboard chords, Compose/IME, lock-screen auto-pause, mouse wheel/motion.


## 0.2.0-alpha.2 acceptance checks (physical hardware required)

- Linux all-keyboards mode: start with no USB keyboard, connect one, type Ctrl+C, disconnect, connect a different model; capture and OBS must continue without another authorization.
- Selected mode: select two keyboards, ensure a third is ignored. Reconnect a selected keyboard with a changed event number; it must resume. With no serial, use the same USB port.
- Hold Ctrl while unplugging, then type a plain letter on another keyboard: no stuck Ctrl label. Stop while no devices remain; helper and broker must exit. Killing the GUI must stop them within three seconds.
- Test a combined keyboard/mouse device: enabling mouse must not enable that device's keyboard if unselected. Leave mouse disabled and confirm no buttons are transmitted.
- Mouse on: left/right/middle clicks, quick taps, held buttons, simultaneous keyboard shortcut, disconnect while held. Buttons must light visibly, then clear; stopping must remove everything.
- Windows: launch extracted ZIP without MSYS2/Rust installed. Test FR AZERTY and US QWERTY foreground layouts, AltGr, Ctrl+Alt+F12, USB unplug/replug and stop/start. Secure/UAC desktops are outside scope.
- Windows halo: full selected-monitor capture plus Browser Source with matching bounds; test 100%, 150%, 200% DPI and a secondary display. No ring should appear for clicks outside the selected monitor. Cropped/window capture is unsupported.

Record OS, keyboard model, selected mode, DPI, OBS source dimensions and result. Do not include sensitive captured text.


## 0.2.0-alpha.3 — multi-monitor acceptance

- Select DISPLAY2, then DISPLAY3 during capture. Test monitors left of / above the primary (negative coordinates) and a portrait monitor.
- Align the selected monitor capture and OBS browser viewport exactly, including aspect ratio. Verify clicks at the center and four corners; clicks outside the selected display must show no ring.
- Mix 100%, 150% and 200% display scaling. Change resolution, orientation and primary monitor during capture; the selected device must remain selected and coordinates follow its current geometry.
- Unplug the selected monitor: the UI must report unavailable and suspend the ring, without affecting keyboard or mouse-button feedback. Reconnect it, or explicitly select another available display.
- Unit tests cover negative origins, monitor boundaries, changing dimensions, monitor order changes and missing selections. The CI runner's actual monitor enumeration is also tested; it does not replace a physical multi-monitor OBS test.
