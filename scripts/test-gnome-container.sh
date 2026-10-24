#!/bin/sh
# Disposable CI container only; exercises a real GNOME 48 nested Wayland compositor.
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y --no-install-recommends gnome-shell gnome-session-bin gsettings-desktop-schemas dbus-x11 xwayland xvfb xauth xdotool python3-gi python3-dbusmock gir1.2-gtk-4.0 libgl1-mesa-dri
extension=/usr/share/gnome-shell/extensions/keycast-bridge@jeromelab.fr
mkdir -p "$extension"
cp -R /work/data/gnome-extension/. "$extension/"
glib-compile-schemas --strict "$extension/schemas"
mkdir -p /run/dbus
dbus-daemon --system --fork
# A container has no logind session. Only the login service is mocked; Mutter,
# Wayland rendering, input grabs, screenshots and the extension are real.
python3 -m dbusmock --system -t logind > /tmp/keycast-logind.log 2>&1 &
python3 - <<'PYMOCK'
import dbus,time
bus=dbus.SystemBus()
for attempt in range(50):
    if bus.name_has_owner('org.freedesktop.login1'): break
    time.sleep(0.1)
obj=bus.get_object('org.freedesktop.login1','/org/freedesktop/login1')
dbus.Interface(obj,'org.freedesktop.DBus.Mock').AddSession('c1','seat0',dbus.UInt32(1000),'tester',True)
PYMOCK
useradd -m tester
install -d -o tester -g tester -m 0700 /tmp/keycast-runtime
runuser -u tester -- env XDG_SESSION_ID=c1 XDG_RUNTIME_DIR=/tmp/keycast-runtime LIBGL_ALWAYS_SOFTWARE=1 MUTTER_DEBUG_DUMMY_MODE_SPECS=1280x720 dbus-run-session -- xvfb-run -a -s '-screen 0 1280x800x24' sh -c '
  set -eu
  gsettings set org.gnome.shell disable-user-extensions false
  gsettings set org.gnome.shell enabled-extensions "[\"keycast-bridge@jeromelab.fr\"]"
  gnome-shell --nested --wayland > /tmp/keycast-gnome.log 2>&1 &
  shell_pid=$!
  trap '\''kill "$shell_pid" 2>/dev/null || true; cat /tmp/keycast-gnome.log'\'' EXIT
  python3 /work/scripts/test-gnome.py
'
