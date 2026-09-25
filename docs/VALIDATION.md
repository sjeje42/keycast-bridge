# Alpha validation report — 2026-09-25

## Passed in the development environment

- `npm ci`, Svelte/TypeScript checking: zero errors and warnings.
- Vite production build: successful; CSS/JS bundled locally, no CDN dependency.
- `cargo check --locked` including the GTK4 GUI, using extracted Ubuntu GTK 4.14 development libraries.
- `cargo clippy --locked --all-targets -- -D warnings`, including GTK4: successful.
- `cargo fmt --check`: successful.
- `cargo test --locked --no-default-features`: **7 tests passed**.
- Native demo executable compiled and ran.
- HTTP/WebSocket integration: valid overlay, invalid token / foreign Origin rejected, configuration and synthetic shortcut received.
- Chromium 134 headless: actual Svelte overlay rendered; background transparent; shortcut expired; no browser errors. Screenshot: `overlay-preview.png`.

## Not validated here

- Running the GTK window in an actual graphical session; only compiler/type checking and linting were performed for this binary.
- A complete linked/release GTK executable on Debian 13.
- Actual evdev capture, privilege drop with Polkit, unplug/replug, or live emergency shortcut on physical hardware.
- OBS Browser Source availability and behavior in the user's Debian package.
- GNOME / KDE / wlroots cross-compositor compatibility.
- GitHub Actions: workflow provided but repository not yet created or pushed.

The source is a reviewable alpha, not a tested production release or a ready-to-install binary package. Follow `TESTING.md` for real-device acceptance before live use.
