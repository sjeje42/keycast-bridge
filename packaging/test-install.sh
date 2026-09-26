#!/bin/sh
# Run in a fresh Debian container: dependencies must come only from APT.
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y --no-install-recommends /work/dist/*.deb xvfb xauth dbus-x11 binutils
# Check runtime linking, packaged paths and authorization policy.
for binary in /usr/bin/keycast-bridge /usr/bin/keycast-bridge-demo /usr/libexec/keycast-bridge-capture; do
    test -x "$binary"
    if ldd "$binary" | grep -q 'not found'; then exit 1; fi
done
strings /usr/bin/keycast-bridge | grep -q /usr/libexec/keycast-bridge-capture
grep -q 'Exec=/usr/bin/keycast-bridge' /usr/share/applications/fr.jeromelab.KeycastBridge.desktop
grep -q /usr/libexec/keycast-bridge-capture /usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy
# A non-root GUI must stay alive under a virtual display, without capturing.
useradd -m tester
set +e
runuser -u tester -- timeout 8s dbus-run-session -- xvfb-run -a /usr/bin/keycast-bridge
result=$?
set -e
[ "$result" -eq 124 ] || { echo "GUI exited unexpectedly: $result"; exit 1; }
apt-get remove -y keycast-bridge
test ! -e /usr/bin/keycast-bridge
test ! -e /usr/libexec/keycast-bridge-capture
test ! -e /usr/share/polkit-1/actions/fr.jeromelab.KeycastBridge.policy
# Reinstallation checks the same user-facing install command again.
apt-get install -y --no-install-recommends /work/dist/*.deb
