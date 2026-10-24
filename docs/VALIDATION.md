# Validation — 0.2.0

This page separates reported manual desktop testing from automated checks. It records the scope of the current stable candidate; [TESTING.md](TESTING.md) is the reusable regression procedure, not a list of tests already passed.

## Manual desktop validation

Status confirmed by the maintainer on 2026-09-28, following development testing of the alpha series:

| Platform | Evidence | Scope and limits |
| --- | --- | --- |
| Debian 13 / GNOME | Successful real keyboard capture and OBS use reported | Does not establish support for every Linux desktop/compositor |
| Fedora | Successful manual desktop use reported | Fedora release, desktop/compositor and hardware were not recorded |
| Windows 10 x64 | Successful manual desktop use reported | Exact Windows build and hardware were not recorded |
| Windows 11 x64 | Successful manual desktop use reported | Exact Windows build and hardware were not recorded |

Development feedback also confirmed multi-monitor use, held modifiers with mouse input, appearance/settings changes, and the alpha.7 fixes for Windows help opening and the single-monitor click ring. These reports establish successful use in the tested setups. They are not a per-platform pass for every scenario in the regression procedure.

On 2026-10-24 the maintainer reported positive extended field testing by friends who deliver training, and chose to leave alpha. These reports do not constitute a manual acceptance test of the new ring controls. New ring controls are covered by preference/protocol/browser regression tests and the Windows GUI smoke test; manual visual feedback on the new effects is still welcome.

## Automated validation

Historical baseline: the icon integration was validated before the stable version work: the following pipelines passed for source commit [`0af3929`](https://github.com/sjeje42/keycast-bridge/commit/0af3929b8e2c8e7b2152a997a758deb423073b8f) on 2026-09-29. The release commit must pass the same three gates and the new GNOME 48 gate before publication.

| Pipeline | Environment | Evidence |
| --- | --- | --- |
| Build and test | Ubuntu 24.04 | [Successful run](https://github.com/sjeje42/keycast-bridge/actions/runs/36534231973) |
| Debian 13 package | Debian 13 container | [Successful run](https://github.com/sjeje42/keycast-bridge/actions/runs/36534231956) |
| Windows portable | Windows Server 2022 CI runner | [Successful run](https://github.com/sjeje42/keycast-bridge/actions/runs/36534232011) |

Coverage includes Rust tests and Clippy, browser overlay regressions, HTTP/WebSocket access checks, synthetic Linux hotplug and privilege separation, Debian install/GUI/remove/reinstall, and Windows portable build/GUI/HTTP checks. The Windows launch check hides the build runtime to verify DLL independence. This runner is not a Windows 10/11 desktop acceptance test.

Regression coverage includes monitor mapping and removal, single-monitor selection, mouse/ring settings, held modifiers, persistent position/colors/canvas dimensions, scaling and pointer placement, settings-window behavior and embedded guides. Package checks cover the bundled HTML and PDF documentation, installed Linux icons, GTK icon-resource lookup, and the Windows executable's embedded icon and Cargo-derived product/version metadata.

Stable packages retain the official icon and PDF guides. Earlier releases retain their original binaries. Current ring coverage includes preference migration and persistence, independent colors/dimensions, all effect combinations, live updates, scaling, expiry and disconnect cleanup.

## Remaining coverage and operating limits

- Fedora release/compositor, exact Windows builds, device models and a per-scenario manual test record still need to be recorded for reproducibility.
- Ubuntu desktop capture, Linux Mint, Manjaro/Arch and other Wayland compositors are not manually validated by the reports above.
- Mixed DPI combinations, portrait/negative-origin monitor layouts, USB removal while modifiers are held, unusual browser associations and small/high-DPI settings dialogs remain regression scenarios unless a dated result is recorded.
- Windows click rings require aligned full-monitor capture and OBS Browser Source geometry. Linux Browser Sources support mouse-button feedback without pointer coordinates; the GNOME 48 extension supplies desktop rings through Display Capture. Application canvas settings do not change OBS properties automatically.

Check the intended OBS scene before a live broadcast and stop capture before entering sensitive information. See [SECURITY.md](../SECURITY.md) for the security model.

## Release gate

All four automated pipelines must succeed for the same commit before a new binary release. Keep successful CI run links and record manual tests using the fields in [TESTING.md](TESTING.md). Existing releases retain their original binaries and validation history.

## Desktop tools — stable candidate

The 0.2.0 presentation tools are new work, separate from the positive field-testing report for earlier builds. Native Windows and GNOME 48 integration tests, bounded scene tests and Cairo pixel checks are included. Physical multi-monitor/DPI and OBS acceptance for the new tools remains to be recorded using `TESTING.md`. The stable release must not be approved solely on the older alpha feedback.

### Verified development checkpoint

The four workflows passed for [`11090ba`](https://github.com/sjeje42/keycast-bridge/commit/11090ba56da801c86e5271762b752ad1eb91eb99) on 2026-10-24:

| Pipeline | Evidence |
| --- | --- |
| Rust/Clippy, browser, protocol and hotplug | [Build and test](https://github.com/sjeje42/keycast-bridge/actions/runs/40101641520) |
| Debian package installation and GUI | [Debian 13](https://github.com/sjeje42/keycast-bridge/actions/runs/40101641552) |
| Native Windows modal lifecycle and portable package | [Windows](https://github.com/sjeje42/keycast-bridge/actions/runs/40101641603) |
| Real GNOME 48 nested Wayland tools | [GNOME integration and screenshots](https://github.com/sjeje42/keycast-bridge/actions/runs/40101641619) |

The synthetic GNOME screenshots were visually inspected for all six trails, spotlight, annotation, whiteboard and frozen zoom; no extension JavaScript errors were present in the log. The container mocks only logind and cannot validate physical session locking or mixed-DPI monitors. This checkpoint does not replace the final commit's required CI gates or the pending physical acceptance.
