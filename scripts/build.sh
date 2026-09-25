#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
npm --prefix web ci
npm --prefix web run check
npm --prefix web run build
cargo build --locked --release
cargo test --locked
