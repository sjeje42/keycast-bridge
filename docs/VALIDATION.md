# Validation — 0.2.0-alpha.2

## Local checks completed

- Svelte/TypeScript: zero errors and warnings; Vite production build succeeds.
- Rust Linux tests without GUI: 10 passed, including SCM_RIGHTS descriptor transfer, device identity after event renumbering, keyboard filtering and HTTP authorization.
- Linux Clippy without GUI: no warnings. GTK4 GUI compilation check succeeds using extracted development libraries.
- Windows capture backend cross-compilation check succeeds; native Windows linking and execution are checked by the Windows workflow.

## Automated release gates

Publication requires successful **Build and test**, **Debian 13 package** and **Windows portable** runs for the same commit. Debian tests package installation, virtual-display GUI launch, removal and reinstall. Windows tests keyboard normalization, native build, portable GUI launch without MSYS2 on PATH and HTTP/WebSocket operation.

See the [Actions results](https://github.com/sjeje42/keycast-bridge/actions) for the actual result of each build. Workflow configuration alone is not evidence of passing tests.

## Manual validation still required

Physical USB reconnect, privileged hotplug broker behavior on a desktop, mouse capture, Windows DPI and OBS alignment have not been exercised in this development environment. Follow [TESTING.md](TESTING.md). The user's keyboard/OBS success on the previous 0.1 alpha does not validate these new features. Fedora, Manjaro and other desktops remain unverified.
