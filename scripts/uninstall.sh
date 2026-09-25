#!/bin/sh
set -eu
[ "$(id -u)" = 0 ] || { echo 'Run: sudo ./scripts/uninstall.sh'; exit 1; }
rm -f /usr/local/bin/keycast-bridge /usr/local/bin/keycast-bridge-demo /usr/local/libexec/keycast-bridge-capture /usr/local/share/applications/fr.jeromelab.KeycastBridge.desktop /usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy
