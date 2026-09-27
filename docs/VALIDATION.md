# Validation — 0.2.0-alpha.6

The user validated alpha.5 appearance controls. This update moves detailed controls into a separate Settings window and adds logical canvas size and offline guides.

## Automated checks

- Rust: legacy preferences acquire the default canvas; custom dimensions are bounded and persist, alongside colors/position; capture state survives live settings changes.
- Browser: position/color/modifier regression checks, multiple logical canvas sizes, centered uniform scaling in a mismatched viewport, portrait pointer-ring placement.
- GUI/package smoke checks: main controls fit the normal initial window, gear opens Settings, closing Settings leaves the main window open. Very small displays retain scrolling.
- HTTP smoke: both embedded guides load, wrong tokens are rejected, overlay/WS controls still work.

## Publication gates

All three pipelines must pass for the same commit: Linux Clippy/tests/browser/hotplug/HTTP, Debian package install/GUI/remove/reinstall, and Windows build/tests/portable GUI/HTTP. The Windows launch test hides the build runtime to verify DLL independence.

## Manual follow-up

Verify the native dialog layout with actual desktop scaling and small screens, color chooser/drag interaction, and the real OBS scene. Application canvas settings do not automatically change OBS Browser Source properties. Pointer rings remain Windows-only and require aligned full-monitor geometry. Guides are available in Settings and bundled as offline HTML/Markdown.
