# Validation — 0.2.0-alpha.8

This page separates reported manual desktop testing from automated checks. It records the scope of the current alpha; [TESTING.md](TESTING.md) is the reusable regression procedure, not a list of tests already passed.

## Manual desktop validation

Status confirmed by the maintainer on 2026-09-28, following development testing of the alpha series:

| Platform | Evidence | Scope and limits |
| --- | --- | --- |
| Debian 13 / GNOME | Successful real keyboard capture and OBS use reported | Does not establish support for every Linux desktop/compositor |
| Fedora | Successful manual desktop use reported | Fedora release, desktop/compositor and hardware were not recorded |
| Windows 10 x64 | Successful manual desktop use reported | Exact Windows build and hardware were not recorded |
| Windows 11 x64 | Successful manual desktop use reported | Exact Windows build and hardware were not recorded |

Development feedback also confirmed multi-monitor use, held modifiers with mouse input, appearance/settings changes, and the alpha.7 fixes for Windows help opening and the single-monitor click ring. These reports establish successful use in the tested setups. They are not a per-platform pass for every scenario in the regression procedure.

## Automated validation

The icon integration was validated before the version bump: the following pipelines passed for source commit [`0af3929`](https://github.com/sjeje42/keycast-bridge/commit/0af3929b8e2c8e7b2152a997a758deb423073b8f) on 2026-09-29. The release commit must pass the same three gates again before publication.

| Pipeline | Environment | Evidence |
| --- | --- | --- |
| Build and test | Ubuntu 24.04 | [Successful run](https://github.com/sjeje42/keycast-bridge/actions/runs/36534231973) |
| Debian 13 package | Debian 13 container | [Successful run](https://github.com/sjeje42/keycast-bridge/actions/runs/36534231956) |
| Windows portable | Windows Server 2022 CI runner | [Successful run](https://github.com/sjeje42/keycast-bridge/actions/runs/36534232011) |

Coverage includes Rust tests and Clippy, browser overlay regressions, HTTP/WebSocket access checks, synthetic Linux hotplug and privilege separation, Debian install/GUI/remove/reinstall, and Windows portable build/GUI/HTTP checks. The Windows launch check hides the build runtime to verify DLL independence. This runner is not a Windows 10/11 desktop acceptance test.

Regression coverage includes monitor mapping and removal, single-monitor selection, mouse/ring settings, held modifiers, persistent position/colors/canvas dimensions, scaling and pointer placement, settings-window behavior and embedded guides. Package checks cover the bundled HTML and PDF documentation, installed Linux icons, GTK icon-resource lookup, and the Windows executable's embedded icon and Cargo-derived product/version metadata.

Alpha.8 packages include the official icon and PDF guides. Earlier releases retain their original binaries. The manual desktop reports above predate this release and are not a new manual acceptance pass for alpha.8.

## Remaining coverage and operating limits

- Fedora release/compositor, exact Windows builds, device models and a per-scenario manual test record still need to be recorded for reproducibility.
- Ubuntu desktop capture, Linux Mint, Manjaro/Arch and other Wayland compositors are not manually validated by the reports above.
- Mixed DPI combinations, portrait/negative-origin monitor layouts, USB removal while modifiers are held, unusual browser associations and small/high-DPI settings dialogs remain regression scenarios unless a dated result is recorded.
- Windows click rings require aligned full-monitor capture and OBS Browser Source geometry. Linux supports mouse-button feedback but not pointer-position rings. Application canvas settings do not change OBS properties automatically.

This is alpha software. Check the intended OBS scene before a live broadcast and stop capture before entering sensitive information. See [SECURITY.md](../SECURITY.md) for the security model.

## Release gate

All three automated pipelines must succeed for the same commit before a new binary release. Keep successful CI run links and record manual tests using the fields in [TESTING.md](TESTING.md). Existing releases retain their original binaries and validation history.
