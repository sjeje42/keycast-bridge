# Desktop presentation tools

Open **Settings → Presentation** and enable **Desktop presentation tools**. They start **off** whenever Keycast Bridge opens. The tools work independently of keyboard capture. **Stop** in the main window also stops the desktop tools.

On Windows 10/11, no additional installation is needed. On Debian 13 with GNOME 48/Wayland, enable the included **Keycast Bridge presentation tools** GNOME extension. After installing the Debian package, log out and back in, then run:

```sh
gnome-extensions enable keycast-bridge@jeromelab.fr
```

For a source installation, run `./scripts/install-gnome-extension.sh` as your ordinary desktop user, then log out/in and enable the extension. Other compositors and GNOME versions are not yet supported for these desktop tools; the existing keyboard overlay still works independently. Disabling the extension immediately removes its effects. Close Keycast Bridge before uninstalling the package.

## Pointer effects

- **Ring:** choose color, radius (10–200 px), thickness (1–20 px), neon and pulse in **Settings → Ring**. Enable the continuous pointer ring in **Presentation**. A short ring also marks clicks. The same appearance settings apply to the optional OBS click ring.
- **Trail:** choose Solid, Dash, Dot, Glow, Particle or Star. Trail color follows the ring settings; line styles also use its thickness; samples fade within 0.65 seconds.
- **Shake to locate:** rapidly move the pointer back and forth several times. A larger ring appears briefly. Fast movement in a straight line does not trigger it.
- **Spotlight:** darken the current monitor around an illuminated **circle or square**. Choose the radius (40–600 px; half the side length for a square) and dimming (0–90%).

Passive effects follow the pointer onto each connected monitor and allow normal interaction with applications. Windows uses physical screen pixels; GNOME uses logical desktop pixels, so desktop scaling can change the apparent physical size.

## Drawing and whiteboard

Move the pointer onto the desired monitor, then press **Ctrl+Alt+D** to annotate or **Ctrl+Alt+B** for a white background. The buttons in Settings open these modes on the monitor currently under the pointer. Choose pen color and width (1–40 px) in Presentation.

| Action | Control |
| --- | --- |
| Draw | Hold the left mouse button and move |
| Undo the last stroke | Ctrl+Z |
| Erase all strokes | Backspace |
| Exit the mode and erase drawings | Esc |
| Stop all desktop tools | Ctrl+Alt+F12 |

These modes take mouse and keyboard input while active. The target application does not receive the drawing clicks. The monitor is fixed until you leave the mode. Drawings are temporary: there is no export or automatic saving. At most 128 strokes with bounded point counts are retained; very long strokes are simplified.

## Frozen zoom

Place the pointer on the desired monitor and press **Ctrl+Alt+Z**, or click **Zoom**. Keycast Bridge takes an in-memory snapshot of that monitor and enlarges it. **This is a frozen image, not live magnification**: animations and application updates do not refresh until you exit and enter zoom again.

Move the pointer to pan; use the mouse wheel or **+ / −** to change magnification (1.25×–8×). The initial factor is configurable. **Esc** returns to the desktop and discards the snapshot. Drawing is available in the separate annotation/whiteboard modes.

## OBS, shortcuts and session changes

Use OBS **Display Capture** (Windows) or **Screen Capture / PipeWire** (GNOME) to include these desktop effects. Window Capture and the keyboard Browser Source alone do not include them. If you capture the desktop ring, turn off the separate **Click ring** option for the OBS Browser Source to avoid displaying two rings.

The desktop shortcuts are registered only while tools are enabled. If Windows reports a shortcut conflict, close or reconfigure the conflicting utility before enabling them again. GNOME shortcuts can be changed through the extension's GSettings schema. Keys reserved by the compositor or another application may conflict.

Closing the application, disabling the tools, locking the session or changing the monitor layout clears the desktop effects. GNOME also stops them when Overview opens. They must be explicitly enabled again. Losing the controlling application clears the GNOME extension within two seconds; losing Windows focus leaves the active drawing/zoom mode. The underlying keyboard-capture mode retains its existing security limitations when presentation tools are off: stop capture before entering secrets.

Effects are limited to ordinary desktop sessions; protected content, secure desktops and exclusive fullscreen applications may not appear or allow overlays. No cursor history, drawing or zoom image is written to disk or sent to the OBS server. Only appearance preferences are saved.
