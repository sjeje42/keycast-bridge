# Keycast Bridge

Online guides: [English](docs/USER_GUIDE.en.md) · [Français](docs/USER_GUIDE.fr.md)

PDF guides: [English](docs/pdf/Keycast_Bridge_User_Guide_EN.pdf) · [Français](docs/pdf/Keycast_Bridge_Guide_Utilisateur_FR.pdf)

**English** · [Français](README.fr.md)

[![Rust stable](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)](Cargo.toml)
[![GTK4](https://img.shields.io/badge/GTK-4-7FE719?logo=gtk&logoColor=white)](Cargo.toml)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white)](web/package.json)
[![Linux Debian 13](https://img.shields.io/badge/Linux-Debian_13-A81D33?logo=debian&logoColor=white)](docs/TESTING.md)
[![Windows 10 / 11 portable x64](https://img.shields.io/badge/Windows-10%20%2F%2011%20portable%20x64-0078D4)](docs/WINDOWS.md)
[![Wayland](https://img.shields.io/badge/Wayland-native-F0C674)](README.md#features)
[![OBS Browser Source](https://img.shields.io/badge/OBS-Browser_Source-302E31?logo=obsstudio&logoColor=white)](README.md#obs-setup)
[![Build and test](https://github.com/sjeje42/keycast-bridge/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sjeje42/keycast-bridge/actions/workflows/ci.yml)
[![License GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue)](LICENSE)

**Display keyboard shortcuts, held modifiers and mouse clicks in OBS on Linux / Wayland and Windows.**

## Quick install

[Download version 0.2.0-alpha.7](https://github.com/sjeje42/keycast-bridge/releases/tag/v0.2.0-alpha.7)

- **Windows 10/11 x64:** download the ZIP, extract it completely and launch `keycast-bridge.exe`.
- **Debian 13 amd64:** download the `.deb`, run `sudo apt install ./keycast-bridge_0.2.0.alpha.7-1_amd64.deb`, then open **Keycast Bridge** from the applications menu.

Windows executables are unsigned; SmartScreen may display a warning. See [antivirus guidance and SHA256 verification](docs/WINDOWS.md#smartscreen-antivirus-and-integrity).

## Overview

[Documentation française](README.fr.md) · [Security model](SECURITY.md) · [Testing](docs/TESTING.md) · [Documentation index](docs/README.md) · [Changelog](CHANGELOG.md)

Version **0.2.0-alpha.7**. New implementation, not a Screenkey fork. GPL-3.0-only.
Rust capture and server, native GTK4 controls (English / French), Svelte + TypeScript browser overlay.
Manually tested on Debian 13 (GNOME), Fedora, Windows 10 and Windows 11. Fedora release and desktop details were not recorded. This does not establish compatibility with every Wayland compositor. See [validation scope and evidence](docs/VALIDATION.md).

![OBS overlay preview](docs/overlay-preview.png)

See the [exact validation status](docs/VALIDATION.md).

## Windows portable (x64)

[Windows portable instructions](docs/WINDOWS.md): extract the complete x64 ZIP and launch `keycast-bridge.exe`; no developer tools or administrator rights required. Windows captures all session keyboards and follows the foreground layout.

## Capture on Linux and Windows

Linux defaults to all keyboards, with optional multiple selection and udev hotplug. A privileged opener passes evdev descriptors over a private Unix socket to a reader that drops privileges. Reconnecting a selected keyboard uses its serial/name or physical USB port/name, never a remembered event number. Without a serial, use the same port. Modifiers are tracked per keyboard: perform a shortcut on one keyboard.

Optional left/right/middle mouse feedback is available on both systems. On Windows, enable **Click ring** before starting; this also enables mouse capture. A sole monitor is selected automatically, including after another monitor is unplugged. With multiple monitors, choose the one captured in OBS under **Settings → Capture**. Align the monitor capture and browser source in OBS. The ring is drawn in OBS, not on the desktop. Linux/Wayland has no pointer-position halo because raw evdev motion does not provide compositor coordinates. See the [Windows limitations](docs/WINDOWS.md).

## Features

- Explicit start and stop, with **Ctrl+Alt+F12** as an emergency stop shortcut. Linux requests Polkit authorization; Windows runs as a normal user.
- Linux capture uses udev hotplug, multiple keyboards, a privileged descriptor broker and an unprivileged reader.
- Linux: AZERTY FR, QWERTY US / UK and QWERTZ DE using libxkbcommon. Select the same layout as your desktop. Windows follows the foreground window’s layout.
- Default shortcuts mode: Ctrl / left Alt / Super combinations, function and navigation keys. Held Shift, Ctrl, Alt, Win/Super and AltGr are shown even without another key. Ordinary text and text entered with AltGr remain filtered by default. Single-letter app shortcuts need the optional all-keys mode.
- Transparent OBS Browser Source, freely positioned overlay, custom colors, adjustable key size/lifetime and configurable canvas dimensions. Position, colors and canvas dimensions persist between launches.
- No key logs, analytics, remote resources, auto-start or background capture after closing the app.
- Read-only overlay URL with a fresh random token per launch, bound to `127.0.0.1:48732` only.

## Debian 13 package (amd64)

Download the `.deb` from [GitHub Releases](https://github.com/sjeje42/keycast-bridge/releases), then open a terminal in the download directory:

```sh
sudo apt install ./keycast-bridge_0.2.0.alpha.7-1_amd64.deb
```

Launch **Keycast Bridge** from the applications menu. No compilation required. APT installs dependencies; the package includes the capture helper and Polkit policy. Capture never starts automatically. Uninstall with `sudo apt remove keycast-bridge`.

Targets **Debian 13 on Intel/AMD 64-bit PCs**. Other Debian versions and Ubuntu are not validated. Uninstall any previous source installation with its uninstall script before installing the package. A manually installed helper alone in `/usr/local/libexec` can remain: the package uses `/usr/libexec`.

**OBS:** Debian’s OBS package has no Browser Source. Use the [official OBS Flatpak](https://obsproject.com/kb/linux-installation), which includes it. Keycast Bridge itself remains a Debian package.

## Platform validation

Manual desktop tests and automated CI are recorded separately in [VALIDATION.md](docs/VALIDATION.md).

| Platform | Method | Validation |
| --- | --- | --- |
| Debian 13 | Package above or source build | Capture/OBS confirmed on GNOME; package installation tested in a container |
| Ubuntu 24.04 LTS and later | Build with APT dependencies | Automated builds and tests on Ubuntu 24.04; real desktop capture still needs testing |
| Linux Mint 22.x (Ubuntu 24.04 base) | Same procedure as Ubuntu | Real desktop testing pending |
| Fedora | Build with DNF dependencies | Manual desktop use confirmed; release and desktop environment not recorded |
| Up-to-date Manjaro / Arch Linux | Build with Pacman dependencies | Proposed instructions, not yet tested on these distributions |
| Windows 10 x64 | Portable ZIP | Manual desktop use confirmed; exact OS build not recorded |
| Windows 11 x64 | Portable ZIP | Manual desktop use confirmed; exact OS build not recorded |

The provided `.deb` remains intended for Debian 13. Use the source instructions below for other distributions. Compatibility with every Wayland compositor has not yet been validated.

## Install from source — other Linux distributions

This method compiles Keycast Bridge on the target machine without creating a package. Requires **GTK 4.8 or newer**, current stable Rust, Node.js 22 and npm. Commands below assume Bash.

### 1. Install your distribution’s dependencies

**Debian 13 / Ubuntu 24.04+ / Linux Mint 22.x:**

```sh
sudo apt update
sudo apt install build-essential pkg-config git curl ca-certificates libgtk-4-dev libxkbcommon-dev libudev-dev xkb-data pkexec polkitd xdg-utils
```

**Fedora Workstation (traditional DNF installation):**

```sh
sudo dnf install gcc gcc-c++ make pkgconf-pkg-config git curl ca-certificates gtk4-devel libxkbcommon-devel systemd-devel xkeyboard-config polkit xdg-utils
```

These instructions do not cover Fedora Silverblue/Kinoite or other immutable variants.

**Manjaro / Arch Linux:**

```sh
sudo pacman -Syu --needed base-devel pkgconf git curl ca-certificates gtk4 libxkbcommon systemd xkeyboard-config polkit xdg-utils
```

This also updates the system to avoid partial upgrades. If the system update requires a reboot, reboot before continuing.

### 2. Set up Rust and Node.js

Install [Rust with rustup](https://rust-lang.org/tools/install/) as your ordinary user. If rustup is already installed, skip directly to the update command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
rustup update stable
```

To use **Node.js 22 with npm**, matching GitHub tests, install [nvm](https://github.com/nvm-sh/nvm#installing-and-updating) if needed, without `sudo`:

```sh
curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.8/install.sh | bash
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "$HOME/.nvm" || printf %s "$XDG_CONFIG_HOME/nvm")"
. "$NVM_DIR/nvm.sh"
nvm install 22
nvm use 22
```

Skip nvm setup if Node.js 22 and npm are already available. Check the tools:

```sh
rustup run stable rustc --version
node --version
npm --version
pkg-config --modversion gtk4
command -v pkexec
```

### 3. Download, compile and install

```sh
git clone https://github.com/sjeje42/keycast-bridge.git
cd keycast-bridge
RUSTUP_TOOLCHAIN=stable sh scripts/build.sh
sudo sh scripts/install.sh
```

Alternatively, download **Code → Download ZIP**, extract it and open a terminal in the directory containing `Cargo.toml`. Use the complete source archive, including `scripts/` and `data/`.

Build **without sudo**. The installation script places executables in `/usr/local/bin`, the capture helper in `/usr/local/libexec`, and installs the launcher and Polkit policy. Installation is required: running the compiled GUI alone does not install the capture helper.

If the Debian package is already installed, remove it with `sudo apt remove keycast-bridge` before installing from source: both methods share the Polkit policy.

### 4. Launch and configure OBS

Open **Keycast Bridge** from your applications menu, or run `/usr/local/bin/keycast-bridge`, without sudo. A minimal graphical session must have a running Polkit authentication agent to display the authorization prompt.

OBS must offer **Sources → + → Browser**. If missing, follow the [official OBS Linux instructions](https://obsproject.com/kb/linux-installation) to install the official Flatpak. Keycast Bridge itself can remain installed natively. See the detailed OBS setup below.

### Update or remove a source installation

Close Keycast Bridge before reinstalling. From an unmodified Git clone:

```sh
git pull --ff-only
RUSTUP_TOOLCHAIN=stable sh scripts/build.sh
sudo sh scripts/install.sh
```

For ZIP downloads, download fresh sources and repeat the build/install steps. To remove **a source installation**, run from the source directory:

```sh
sudo sh scripts/uninstall.sh
```

For **the Debian package**, use only `sudo apt remove keycast-bridge`, not the script.

Dependency names and package-manager commands: [Ubuntu](https://packages.ubuntu.com/noble/libgtk-4-dev), [Fedora GTK4](https://packages.fedoraproject.org/pkgs/gtk4/gtk4-devel/), [Fedora libxkbcommon](https://packages.fedoraproject.org/pkgs/libxkbcommon/libxkbcommon-devel/), [Arch GTK4](https://archlinux.org/packages/extra/x86_64/gtk4/), [Manjaro Pacman](https://wiki.manjaro.org/index.php/Pacman_Overview).

## OBS setup

1. In **Settings → Capture**, Linux users can keep **All keyboards (automatic)** or select specific keyboards and a matching layout. Windows captures all session keyboards and chooses the layout automatically. Enable optional mouse feedback before starting.
2. Copy the OBS URL. In OBS, add **Browser** as a source; match the dimensions chosen in **Settings → Canvas and size** (default **1920 × 1080**). Paste the URL. Keep the page background transparent.
3. Click **Test overlay** to display a synthetic shortcut; no keyboard access is needed. **Open preview** opens the same overlay in your browser.
4. Click **Start** and approve the authorization request on Linux. Check that the status says **Capture active** before recording.
5. Click **Stop**, or press **Ctrl+Alt+F12** on a captured keyboard. The shortcut itself is not displayed.

The URL changes after every app launch. Update it in OBS. This deliberately avoids storing a long-lived token. OBS does not need administrator privileges.

**If Browser is absent from your OBS source list**, that OBS build has no Browser Source support. Install an OBS build/package providing that feature; this alpha does not implement an alternative window overlay.

## Appearance and settings

### Held modifiers and mouse drawing

Shift, Ctrl, Alt and combinations stay visible in a live row while held, including during Pen tool clicks and drags. An outline in the chosen accent color distinguishes held keys from the timed last-shortcut row above. Works on Linux and Windows without enabling text keys. Enable mouse clicks to display buttons alongside held modifiers.

### Position and colors

Open the top-right gear → **Position and colors**, then drag the block in the schematic preview. Nine presets and **X/Y (%)** fields also support keyboard adjustment. 0% means the left/top edge and 100% the right/bottom edge of the available space, with a safety margin. Both rows and the mouse icon move together.

Customize background, key background, text and all three click colors. The accent also colors held-key outlines and the click ring. Light/dark palettes preserve position; **Reset all** restores default position and colors.

Changes apply live in OBS during capture. The GTK preview is schematic; **Test overlay** shows exact output in OBS but stops active capture. Positions are relative to the Browser Source; cropping that source can hide content. The click ring remains at the pointer.

Position, colors and canvas dimensions persist in `%APPDATA%\keycast-bridge\appearance.json` on Windows and `$XDG_CONFIG_HOME/keycast-bridge/appearance.json` or `~/.config/keycast-bridge/appearance.json` on Linux. No keystrokes or OBS token are stored. Other options remain session-only. Save failures are shown in the interface.

### Settings and OBS canvas

Open the top-right gear for **Position and colors**, **Canvas and size**, and **Capture**. The main window keeps recording controls visible; settings use a separate window. Choose standard or custom canvas dimensions (160–7680 px per side) and enter the same values in OBS Browser Source properties. A different browser viewport is fitted uniformly with transparent margins. Canvas dimensions join position/colors in saved preferences. **Complete guide** opens the embedded offline help; HTML/Markdown copies remain included in packages. On Windows, help and preview use the default browser; if opening fails, a dialog offers **Copy link**. Keep Keycast Bridge open while reading its local help page. The illustrated PDFs are also available separately in the release downloads and in `docs/pdf/`; future package builds include them alongside the HTML guides.

## Development and demo

```sh
npm --prefix web ci
npm --prefix web run build
cargo test --locked --no-default-features
cargo run --locked --no-default-features --bin keycast-bridge-demo
```

The demo prints a local URL and sends synthetic shortcuts every three seconds. It does not open any input device. Only one app/demo can use port 48732 at a time.

The generated web assets are embedded into Rust binaries; build the web frontend before Cargo. Run `npm --prefix web run check` and `cargo fmt --check` before committing. CI builds the GTK app, checks formatting and linting, and runs unit and HTTP/WebSocket integration tests. The badge above links to the current workflow status.

## Current limits

- No automatic password-field detection: stop capture before entering secrets, even in shortcuts-only mode.
- Linux hotplug continues without restarting. A selected keyboard without a serial number must return to the same USB port.
- Layout changes in GNOME are not automatically tracked; stop, choose the new layout, then restart.
- No text reconstruction, Compose/dead-key composition, IME, mouse wheel/motion visualization. Appearance and canvas settings persist; size, duration, capture options, monitor selection and language remain session-only.
- Key-repeat events are intentionally ignored. Modifiers held before Start must be released and pressed again. Caps Lock / Num Lock state already active at startup is not synchronized with GNOME.
- System-wide evdev access does not identify focused windows, lock screens or active sessions. Stop before locking the session; lock-screen auto-pause is not implemented.
- Manual desktop testing is confirmed on Debian 13, Fedora, Windows 10 and Windows 11. This remains an alpha: check your own OBS scene before going live. Untested configurations and regression checks are listed in [VALIDATION.md](docs/VALIDATION.md) and [TESTING.md](docs/TESTING.md).

## License and acknowledgments

Copyright © 2026 Jérôme Stavrianos. Licensed under **GPL-3.0-only**, see [LICENSE](LICENSE). No Screenkey source code or assets are included. Screenkey inspired the use case. Dependencies retain their own licenses.
