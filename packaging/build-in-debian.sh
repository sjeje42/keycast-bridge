#!/bin/sh
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y --no-install-recommends build-essential curl ca-certificates pkg-config libgtk-4-dev libxkbcommon-dev xkb-data dpkg-dev python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/keycast-rustup.sh
sh /tmp/keycast-rustup.sh -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
export KEYCAST_HELPER_PATH=/usr/libexec/keycast-bridge-capture
cargo test --locked --no-default-features
cargo build --locked --release
sh packaging/build-deb.sh
