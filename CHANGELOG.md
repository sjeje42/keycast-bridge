# Changelog

Notable changes, newest first. Dates refer to GitHub release publication. All versions below are prereleases. [Current release notes](packaging/RELEASE.md) provide the bilingual installation summary; this file preserves earlier releases.

## Unreleased

No changes yet.

## [0.2.0-alpha.8](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.8) — 2026-09-29

### Changed

- Add bilingual quick installation instructions, Windows portable badges, and unsigned-build/SmartScreen guidance with release-specific ZIP checksum verification.
- Update repository description and topics for Windows support; enable Dependabot alerts.
- Keep appearance/help security details before the private reporting procedure.

- Document reported manual testing on Debian 13, Fedora, Windows 10 and Windows 11 separately from automated CI coverage.
- Refresh the regression procedure, including persistent preferences and current multi-monitor behavior.
- Rename the bilingual documentation index to `docs/README.md`.
- Derive the Windows ZIP name and checksum entry from `Cargo.toml`; check current manifest/document versions in CI and document Debian version conversion.
- Ignore generated distributions/ZIPs, local environment files and editor configuration.
- Enable GitHub private vulnerability reporting and provide a direct reporting link in the security policy.

### Added

- Official application icon: preserved 2048px artwork, multi-resolution Windows ICO and embedded executable resources, GTK icon resources, and Linux launcher icons up to 1024px. Windows product/version metadata follows `Cargo.toml`.
- Preparatory code signing policy with maintainer roles, the intended Windows executable scope, privacy details and pending SignPath integration tasks; current releases remain unsigned.

- Illustrated French and English PDFs in the repository and as separate alpha.7 release downloads; preserve online and embedded HTML guides. Alpha.8 packages bundle the PDFs. Existing alpha.7 binaries are unchanged.
- Historical changelog and release-maintenance checklist.

### Fixed

- Debian slim-container package checks now explicitly retain the package documentation before comparing installed HTML/PDF files.

## [0.2.0-alpha.7](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.7) — 2026-09-28

### Fixed

- Open Windows help and preview through the native default-browser launcher; show an error and Copy link fallback if dispatch fails.
- Enabling the click ring also enables the required mouse capture.
- Automatically select a sole monitor, including after disconnecting a second display; preserve manual selection while multiple displays remain.
- Keep the active overlay state intact during unchanged single-monitor refreshes.

## [0.2.0-alpha.6](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.6) — 2026-09-27

### Added

- Separate settings window opened with a gear button, with appearance, canvas and capture tabs.
- Saved standard/custom canvas dimensions, 160–7680 pixels per side, proportional preview and OBS dimension hint.
- Complete offline English and French guides in HTML/Markdown.

### Changed

- Compact main window with recording controls kept visible.
- Uniform overlay scaling with transparent margins for mismatched browser dimensions.

### Fixed

- Keep the animated click ring centered in the scaled canvas.

## [0.2.0-alpha.5](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.5) — 2026-09-27

### Added

- Freely positioned overlay through a draggable schematic preview, nine presets and numeric coordinates.
- Six customizable colors, light/dark palettes and appearance reset.
- Persistent position and colors, applied live without stopping capture.

## [0.2.0-alpha.4](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.4) — 2026-09-27

### Added

- Live held Shift/Ctrl/Alt feedback during clicks and drags, separate from the timed shortcut row, on Linux and Windows.
- Distinct AltGr handling and left/right modifier state; per-keyboard state on Linux.

### Fixed

- Clear stale held state on removal/stop/disconnect and restore current modifiers when OBS reconnects.

## [0.2.0-alpha.3](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.3) — 2026-09-27

### Added

- Windows monitor selector for the OBS click ring, with resolution, position and primary-display information.
- Coordinate mapping for negative origins, portrait displays and changing resolutions.
- Automatic display-list refresh and live monitor selection; suspend the ring when its selected display disappears. Alpha.7 later adds automatic selection when only one display remains.

## [0.2.0-alpha.2](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.2) — 2026-09-27

### Added

- Portable Windows x64 build with bundled GTK runtime, without administrator rights or developer tools.
- Linux udev hotplug, all-keyboard capture by default or multiple selected keyboards, with privileged device opening separated from the unprivileged event reader.
- Optional left/right/middle mouse-button feedback and a Windows primary-monitor click ring.

## [0.1.0-alpha.2](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.1.0-alpha.2) — 2026-09-26

### Added

- Ready-to-install Debian 13 amd64 package, including capture helper, Polkit policy and dependencies.
- Automated clean-container installation, GTK launch, removal and reinstallation checks.
- Bilingual Linux interface, authorized keyboard capture and transparent OBS overlay.
