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
- [ ] Close app / disconnect keyboard: capture ends, OBS clears. No helper remains.
- [ ] Restart app: old URL fails, new URL works; preferences intentionally reset.
- [ ] OBS reconnect / hidden source / slow consumer: no stale key replay.
- [ ] Long session: no persistent key logs or increasing memory backlog.

**Not guaranteed in this alpha:** initial Caps/Num Lock state, automatic desktop layout tracking, multi-keyboard chords, Compose/IME, lock-screen auto-pause, mouse input.
