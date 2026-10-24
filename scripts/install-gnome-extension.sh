#!/bin/sh
# Install as the desktop user, without sudo. Activation remains an explicit choice.
set -eu
cd "$(dirname "$0")/.."
[ "$(id -u)" != 0 ] || { echo 'Run as your desktop user, without sudo.'; exit 1; }
extension="${XDG_DATA_HOME:-$HOME/.local/share}/gnome-shell/extensions/keycast-bridge@jeromelab.fr"
install -d "$extension/schemas"
install -m 0644 data/gnome-extension/*.js data/gnome-extension/metadata.json "$extension/"
install -m 0644 data/gnome-extension/schemas/*.xml "$extension/schemas/"
glib-compile-schemas --strict "$extension/schemas"
printf '%s\n' 'Installed for GNOME 48. Log out and back in, then run:' 'gnome-extensions enable keycast-bridge@jeromelab.fr' 'The tools only run after you enable them in Keycast Bridge.'
