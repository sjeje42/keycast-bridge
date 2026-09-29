#!/bin/sh
set -eu
[ "$(id -u)" = 0 ] || { echo 'Run: sudo ./scripts/uninstall.sh'; exit 1; }
rm -f /usr/local/bin/keycast-bridge /usr/local/bin/keycast-bridge-demo /usr/local/libexec/keycast-bridge-capture /usr/local/share/applications/fr.jeromelab.KeycastBridge.desktop /usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy
rm -f /usr/local/share/icons/hicolor/*/apps/fr.jeromelab.KeycastBridge.png
if command -v gtk4-update-icon-cache >/dev/null 2>&1; then
    gtk4-update-icon-cache -f -t /usr/local/share/icons/hicolor
fi
