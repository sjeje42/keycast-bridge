# Validation — 0.2.0-alpha.5

Previous alpha.4 passed all three pipelines; the user confirmed its held-modifier behavior worked.

## Checks for this update

- Rust tests: sanitized coordinates/colors, saved settings round-trip and replacement, corrupt-file fallback, live configuration preserving active capture and held modifiers.
- Browser tests: all four corners, center and arbitrary positions on landscape and portrait canvases; custom text/background/accent/button colors; pointer halo independent of keyboard placement; invalid settings fallback; modifier and disconnect regression tests.
- Full GTK compilation and lint checks target Linux and Windows.

## Publication gates

All three GitHub pipelines must succeed for the same commit: Linux tests/Clippy/udev hotplug/browser/HTTP checks, Debian package install/launch/remove/reinstall, and Windows tests/build/portable launch/HTTP checks. Portable launch temporarily hides the build runtime directory to catch DLL dependencies.

## Remaining manual validation

Check the native drag preview, color chooser and persistence on physical Linux/Windows desktops, then compare with OBS on actual monitor/DPI setups. The GTK preview is schematic, not a screenshot of OBS. Coordinates refer to the Browser Source viewport; cropping or transforming that source in OBS changes the visible result. Linux/Wayland still has no pointer-position halo.
