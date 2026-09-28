# Validation — 0.2.0-alpha.7

The user validated alpha.6 and reported Windows help launch failure and difficulty enabling a single-monitor click ring. Alpha.7 uses native Windows URL dispatch with a visible failure/copy-link fallback, directly enables the required mouse capture with the ring, and automatically selects a sole connected monitor.

## Automated checks

- Display mapping: sole monitor at startup, secondary-monitor removal, changed display identity, empty enumeration and continued strict selection with multiple monitors.
- Windows GUI smoke: enabling the ring enables mouse capture; disabling mouse capture clears the ring.

- Rust: legacy preferences acquire the default canvas; custom dimensions are bounded and persist, alongside colors/position; capture state survives live settings changes.
- Browser: position/color/modifier regression checks, multiple logical canvas sizes, centered uniform scaling in a mismatched viewport, portrait pointer-ring placement.
- GUI/package smoke checks: main controls fit the normal initial window, gear opens Settings, closing Settings leaves the main window open. Very small displays retain scrolling.
- HTTP smoke: both embedded guides load, wrong tokens are rejected, overlay/WS controls still work.

## Publication gates

All three pipelines must pass for the same commit: Linux Clippy/tests/browser/hotplug/HTTP, Debian package install/GUI/remove/reinstall, and Windows build/tests/portable GUI/HTTP. The Windows launch test hides the build runtime to verify DLL independence.

## Manual follow-up

Verify help/preview opening in the actual default browser and ring alignment in a single-monitor OBS scene, including a transition from two monitors to one. The CI does not simulate a real default browser. Verify the native dialog layout with actual desktop scaling and small screens, color chooser/drag interaction, and the real OBS scene. Application canvas settings do not automatically change OBS Browser Source properties. Pointer rings remain Windows-only and require aligned full-monitor geometry. Guides are available in Settings and bundled as offline HTML/Markdown.
