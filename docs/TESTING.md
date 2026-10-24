# Regression and acceptance test procedure

This procedure applies to the current version in `Cargo.toml`. It is a reusable checklist, not a record of completed tests. Manual use is confirmed on Debian 13, Fedora, Windows 10 and Windows 11; see [VALIDATION.md](VALIDATION.md) for the evidence and remaining coverage gaps. Run applicable Linux checks on Debian and Fedora, and Windows checks on both Windows 10 and 11.

## Automated

```sh
python3 scripts/check-version.py
npm --prefix web ci
npm --prefix web run check
npm --prefix web run build
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

Unit tests cover layout-dependent Ctrl+A/Ctrl+Q, modifier release, ignored repeat events, emergency stop suppression, shortcuts-only filtering, protocol shape, rejected origins/hosts/tokens and clearing on stop. They do not simulate real evdev permissions or Polkit.

The included `scripts/smoke.mjs` exercises a running demo over HTTP and WebSocket (Node.js 22). Build `keycast-bridge-demo` first and ensure port 48732 is free.

See [VALIDATION.md](VALIDATION.md) for checks actually completed for this version.

## Linux desktop + OBS regression checklist (Debian 13 / Fedora)

- [ ] Build, install and launch from GNOME without sudo.
- [ ] UI starts stopped; FR/EN labels and keyboard focus work.
- [ ] OBS includes Browser Source; transparent preview and synthetic demo appear.
- [ ] Wrong token and foreign Origin cannot subscribe; only 127.0.0.1 listens.
- [ ] Cancel Polkit: stopped/error state, no capture. Start again successfully.
- [ ] Check that non-keyboard devices cannot be selected as keyboards. Start in All keyboards mode, then select a real keyboard explicitly: capture active.
- [ ] FR Ctrl+A, Ctrl+C, Shift+Ctrl+V, Alt+Tab, Super+arrows, F1–F12.
- [ ] Switch both desktop and app to US: same physical Q position now yields Ctrl+Q.
- [ ] Default mode does not show unmodified letters or AltGr text.
- [ ] All-keys mode intentionally shows letters; disable after testing.
- [ ] Ctrl+Alt+F12 clears OBS and stops; no emergency chord is displayed.
- [ ] Stop during the authorization dialog, then authorize: no event reaches OBS.
- [ ] Stop/restart rapidly: old capture cannot publish into new session.
- [ ] Close app: capture ends, OBS clears. No helper remains. Disconnecting a keyboard clears its state while capture continues.
- [ ] Restart app: old URL fails, new URL works; position, colors and canvas dimensions persist. Other capture options reset for the new session.
- [ ] OBS reconnect / hidden source / slow consumer: no stale key replay.
- [ ] Long session: no persistent key logs or increasing memory backlog.

**Not guaranteed in this version:** initial Caps/Num Lock state, automatic desktop layout tracking, multi-keyboard chords, Compose/IME, lock-screen auto-pause, mouse wheel/motion.


## Keyboard hotplug and mouse buttons (physical hardware)

- Linux all-keyboards mode: start with no USB keyboard, connect one, type Ctrl+C, disconnect, connect a different model; capture and OBS must continue without another authorization.
- Selected mode: select two keyboards, ensure a third is ignored. Reconnect a selected keyboard with a changed event number; it must resume. With no serial, use the same USB port.
- Hold Ctrl while unplugging, then type a plain letter on another keyboard: no stuck Ctrl label. Stop while no devices remain; helper and broker must exit. Killing the GUI must stop them within three seconds.
- Test a combined keyboard/mouse device: enabling mouse must not enable that device's keyboard if unselected. Leave mouse disabled and confirm no buttons are transmitted.
- Mouse on: left/right/middle clicks, quick taps, held buttons, simultaneous keyboard shortcut, disconnect while held. Buttons must light visibly, then clear; stopping must remove everything.
- Windows: launch extracted ZIP without MSYS2/Rust installed. Test FR AZERTY and US QWERTY foreground layouts, AltGr, Ctrl+Alt+F12, USB unplug/replug and stop/start. Secure/UAC desktops are outside scope.
- Windows halo: full selected-monitor capture plus Browser Source with matching bounds; test 100%, 150%, 200% DPI and a secondary display. No ring should appear for clicks outside the selected monitor. Cropped/window capture is unsupported.

Record the application version/commit, test date, OS release/build, desktop/compositor, keyboard model, selected mode, DPI, OBS version and source dimensions, and a result for each scenario. Do not include sensitive captured text.


## Windows multi-monitor regression checks

- Select DISPLAY2, then DISPLAY3 during capture. Test monitors left of / above the primary (negative coordinates) and a portrait monitor.
- Align the selected monitor capture and OBS browser viewport exactly, including aspect ratio. Verify clicks at the center and four corners; clicks outside the selected display must show no ring.
- Mix 100%, 150% and 200% display scaling. Change resolution, orientation and primary monitor during capture; the selected device must remain selected and coordinates follow its current geometry.
- With three monitors, unplug the selected monitor: report unavailable and suspend its ring while two monitors remain. With two monitors, unplug either one: automatically map clicks to the sole remaining monitor. Keyboard and mouse-button feedback must continue. Leave the single-monitor scene running and verify periodic display refresh does not clear held buttons or shortcuts.
- Unit tests cover negative origins, monitor boundaries, changing dimensions, monitor order changes and missing selections. The CI runner's actual monitor enumeration is also tested; it does not replace a physical multi-monitor OBS test.


## Windows help and single-monitor ring

- Open Complete guide in both languages and Preview with the Windows default browser, from the extracted portable ZIP without MSYS2 installed.
- If no HTTP browser handler is configured, verify the visible failure dialog and Copy link fallback. Keep the app open when pasting the link.
- Start with one monitor and mouse capture unchecked. Enable Click ring: mouse capture must become checked. Start capture and check the ring at the center and corners of the aligned OBS scene.
- Disable mouse capture while stopped: the ring checkbox must clear. During capture without mouse capture, stop before enabling it.

## Held modifiers

Verify Shift/Ctrl/Alt alone, combinations, both left/right keys, release during a drag and holds longer than the overlay duration. Repeat with mouse capture disabled. Confirm AltGr is not displayed as Ctrl+Alt. Reconnect the OBS Browser Source while Shift is held; it must recover current state and clear on release. Stop capture while holding keys: both rows clear. Under Linux, unplug a keyboard with Ctrl held while Shift remains held on another: only Shift remains. Physical Photoshop/Affinity Pen-tool validation is manual.

## Appearance

Drag the schematic preview; test all nine presets and numeric X/Y via keyboard. Change each color, including a three-button mouse test and Windows pointer halo. Change size/duration while using a custom style: position/colors must remain. Reconnect the Browser Source and restart the app: position/colors must return. Light/dark palettes preserve position; reset restores bottom-center/default colors. Verify a failed preference save is reported without interrupting capture. Test landscape/portrait browser viewports and OBS crops; native preview is schematic.

## Settings and canvas

Check initial main-window fit on 1280×720/1366×768 displays and high-DPI scaling. Open the gear, switch all settings tabs (including Click ring on Windows), close/reopen Settings and verify capture continues. Choose a portrait preset and a custom canvas, align OBS width/height, then restart and confirm dimensions persist. With mismatched browser dimensions, verify centered transparent margins and no distortion. Check the main dimension hint, preview aspect and pointer ring. Open the complete guide in each language with the network disconnected; also open the bundled HTML directly.

## Custom click ring (Windows)

In Settings → Click ring, change color, base radius and thickness; try all four neon/pulse combinations. Check the reduced preview and OBS output. Verify a 10px radius caps thickness at 10px; a 200px radius accepts 20px thickness. Change key size and palette: ring dimensions and color must remain independent. Confirm each click fades within 0.5s, a rapid second click restarts the effect, and Stop/disconnect/disable clears it. Change style while capture is active: held modifiers and capture must continue. Restart and confirm all five ring settings persist; test an old preference file without a ring section. Reset ring must preserve other appearance settings. Repeat with portrait/4K canvas, a scaled browser source and multiple monitors. Record actual manual results, not just completion of this checklist.

## Desktop presentation acceptance (0.2.0 candidate)

Automated: `node scripts/test-presentation.mjs` checks bounded scene state and settings. Rust/Cairo tests check transparent circle/square spotlight apertures, all six rendered trail styles and annotation/whiteboard backgrounds. Windows tests create a real layered window, enter each modal tool, send Escape and verify input styles/closure. The GNOME 48 workflow runs a nested Wayland compositor on Debian 13 (only logind is mocked in the container; physical lock/unlock remains a manual check) and exercises drawing, undo, clear, Escape, zoom snapshots, competing clients and heartbeat/connection cleanup.

Before approving the stable release, test on physical Windows 10/11 and Debian 13/GNOME 48:

1. Enable tools; confirm all six trails and ring color/radius/width/neon/pulse, both spotlight shapes and dimming extremes. Move normally and shake; straight fast motion must not trigger the locator.
2. Move between monitors with different resolutions/scaling, including a secondary monitor left of the primary on Windows. Verify normal clicks reach applications under passive effects.
3. Use Ctrl+Alt+D/B/Z on each monitor. Draw multiple strokes; verify Ctrl+Z removes one, Backspace clears, Escape restores input and clears temporary content. Zoom with wheel/+/- and pan to all edges. Check the displayed image is explicitly frozen.
4. Alt-Tab from a Windows modal tool; lock/unlock, unplug a monitor, stop/close/terminate the app, and disable the GNOME extension. No invisible input grab or effect may remain; tools must not reactivate after lock.
5. Record with OBS Display Capture/PipeWire at 1080p and 4K. Confirm cursor alignment, readable neon/trails and no duplicate OBS click ring. Window Capture is not the desktop-effect path.
6. Reopen the app: styles restored, tools off, no strokes/snapshot retained. Check CPU/GPU load during a long tutorial and shortcut conflicts with the actual training software.

These new tools are not covered by the earlier alpha field-test report. Record OS/GNOME version, scaling, display topology and OBS capture method with acceptance results.
