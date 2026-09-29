#!/bin/sh
# Run inside Debian 13 after compiling with KEYCAST_HELPER_PATH below.
set -eu
cd "$(dirname "$0")/.."
. /etc/os-release
[ "$ID:$VERSION_ID" = debian:13 ] || { echo 'Build this package on Debian 13'; exit 1; }
[ "$(dpkg --print-architecture)" = amd64 ] || exit 1
version=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"].replace("-", "~", 1) + "-1")')
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
pkg="$work/package"
install -d "$pkg/DEBIAN" "$pkg/usr/bin" "$pkg/usr/libexec" "$pkg/usr/share/applications" "$pkg/usr/share/polkit-1/actions" "$pkg/usr/share/doc/keycast-bridge"
install -m 0755 target/release/keycast-bridge target/release/keycast-bridge-demo "$pkg/usr/bin/"
install -m 0755 target/release/keycast-bridge-capture "$pkg/usr/libexec/"
for icon in data/icons/hicolor/*/apps/*.png; do
    relative=${icon#data/icons/}
    install -D -m 0644 "$icon" "$pkg/usr/share/icons/$relative"
done
sed 's|/usr/local/bin/|/usr/bin/|g' data/fr.jeromelab.KeycastBridge.desktop > "$pkg/usr/share/applications/fr.jeromelab.KeycastBridge.desktop"
sed 's|/usr/local/libexec/|/usr/libexec/|g' data/fr.jeromelab.KeycastBridge.policy > "$pkg/usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy"
install -m 0644 LICENSE "$pkg/usr/share/doc/keycast-bridge/copyright"
install -m 0644 README.md README.fr.md SECURITY.md "$pkg/usr/share/doc/keycast-bridge/"
install -m 0644 docs/USER_GUIDE.en.md docs/USER_GUIDE.fr.md "$pkg/usr/share/doc/keycast-bridge/"
cp -R docs/guide "$pkg/usr/share/doc/keycast-bridge/guide"
cp -R docs/pdf "$pkg/usr/share/doc/keycast-bridge/pdf"
strip "$pkg/usr/bin/"* "$pkg/usr/libexec/"*
mkdir "$work/debian"
cat > "$work/debian/control" <<'CONTROL'
Source: keycast-bridge
Section: video
Priority: optional
Maintainer: Jérôme Stavrianos <58558882+sjeje42@users.noreply.github.com>

Package: keycast-bridge
Architecture: amd64
Description: Local keyboard shortcut overlay for OBS
CONTROL
deps=$(cd "$work" && dpkg-shlibdeps -O -e"$pkg/usr/bin/keycast-bridge" -e"$pkg/usr/bin/keycast-bridge-demo" -e"$pkg/usr/libexec/keycast-bridge-capture")
deps=${deps#shlibs:Depends=}
[ -n "$deps" ]
cat > "$pkg/DEBIAN/control" <<CONTROL
Package: keycast-bridge
Version: $version
Section: video
Priority: optional
Architecture: amd64
Maintainer: Jérôme Stavrianos <58558882+sjeje42@users.noreply.github.com>
Depends: $deps, pkexec, polkitd, xkb-data, xdg-utils
Installed-Size: $(du -sk "$pkg/usr" | cut -f1)
Homepage: https://github.com/sjeje42/keycast-bridge
Description: Local keyboard shortcut overlay for OBS on Linux and Wayland
 Bilingual GTK4 interface, AZERTY/QWERTY layouts and transparent browser overlay.
 Capture requires explicit administrator authorization and starts disabled.
 Built for Debian 13. OBS must provide Browser Source (official Flatpak).
CONTROL
chmod 0644 "$pkg/DEBIAN/control" "$pkg/usr/share/applications/"* "$pkg/usr/share/polkit-1/actions/"*
mkdir -p dist
# GitHub replaces tildes in asset names; use the final name before checksumming.
filename_version=$(printf '%s' "$version" | tr '~' '.')
dpkg-deb --root-owner-group --build "$pkg" "dist/keycast-bridge_${filename_version}_amd64.deb"
dpkg-deb --info "dist/keycast-bridge_${filename_version}_amd64.deb"
