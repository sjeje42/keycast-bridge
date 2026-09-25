#!/bin/sh
# Build as your ordinary user; only this installation step needs sudo.
set -eu
cd "$(dirname "$0")/.."
[ "$(id -u)" = 0 ] || { echo 'Run: sudo ./scripts/install.sh'; exit 1; }
for binary in keycast-bridge keycast-bridge-capture keycast-bridge-demo; do
    [ -f "target/release/$binary" ] || { echo 'Build first: ./scripts/build.sh'; exit 1; }
done
install -d /usr/local/bin /usr/local/libexec /usr/local/share/applications /usr/share/polkit-1/actions
install -o root -g root -m 0755 target/release/keycast-bridge /usr/local/bin/
install -o root -g root -m 0755 target/release/keycast-bridge-demo /usr/local/bin/
install -o root -g root -m 0755 target/release/keycast-bridge-capture /usr/local/libexec/
install -o root -g root -m 0644 data/fr.jeromelab.KeycastBridge.desktop /usr/local/share/applications/
install -o root -g root -m 0644 data/fr.jeromelab.KeycastBridge.policy /usr/share/polkit-1/actions/
echo 'Installed. Launch Keycast Bridge from GNOME / Installation terminée.'
