#!/bin/sh
# Run in a fresh Debian container: dependencies must come only from APT.
set -eu
export DEBIAN_FRONTEND=noninteractive
# debian:13-slim excludes /usr/share/doc by default. Include this package's
# documentation so the install check validates the files shipped to desktops.
printf '%s\n' 'path-include /usr/share/doc/keycast-bridge/*' > /etc/dpkg/dpkg.cfg.d/zz-keycast-test-docs
apt-get update
apt-get install -y --no-install-recommends /work/dist/*.deb xvfb xauth dbus-x11 binutils
# Check runtime linking, packaged paths and authorization policy.
for binary in /usr/bin/keycast-bridge /usr/bin/keycast-bridge-demo /usr/libexec/keycast-bridge-capture; do
    test -x "$binary"
    if ldd "$binary" | grep -q 'not found'; then exit 1; fi
done
strings /usr/bin/keycast-bridge | grep -q /usr/libexec/keycast-bridge-capture
grep -q 'Exec=/usr/bin/keycast-bridge' /usr/share/applications/fr.jeromelab.KeycastBridge.desktop
grep -q '^Icon=fr.jeromelab.KeycastBridge$' /usr/share/applications/fr.jeromelab.KeycastBridge.desktop
for size in 16 24 32 48 64 128 256 512 1024; do
    icon="hicolor/${size}x${size}/apps/fr.jeromelab.KeycastBridge.png"
    cmp "/work/data/icons/$icon" "/usr/share/icons/$icon"
done
grep -q /usr/libexec/keycast-bridge-capture /usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy
for name in Keycast_Bridge_Guide_Utilisateur_FR Keycast_Bridge_User_Guide_EN; do
    cmp "/work/docs/pdf/$name.pdf" "/usr/share/doc/keycast-bridge/pdf/$name.pdf"
done
for language in en fr; do
    cmp "/work/docs/guide/$language.html" "/usr/share/doc/keycast-bridge/guide/$language.html"
done
# A non-root GUI must stay alive under a virtual display, without capturing.
useradd -m tester
set +e
runuser -u tester -- timeout 8s dbus-run-session -- xvfb-run -a /usr/bin/keycast-bridge
result=$?
set -e
[ "$result" -eq 124 ] || { echo "GUI exited unexpectedly: $result"; exit 1; }
runuser -u tester -- env KEYCAST_SMOKE_TEST=1 dbus-run-session -- xvfb-run -a /usr/bin/keycast-bridge
apt-get remove -y keycast-bridge
test ! -e /usr/bin/keycast-bridge
test ! -e /usr/libexec/keycast-bridge-capture
test ! -e /usr/share/icons/hicolor/1024x1024/apps/fr.jeromelab.KeycastBridge.png
test ! -e /usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy
# Reinstallation checks the same user-facing install command again.
apt-get install -y --no-install-recommends /work/dist/*.deb
