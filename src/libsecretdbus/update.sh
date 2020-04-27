#!/usr/bin/env bash
set -x
set -euo pipefail

curl "https://raw.githubusercontent.com/freedesktop/xdg-specs/master/secret-service/org.freedesktop.Secrets.xml" > ./org.freedesktop.Secrets.xml
cargo install dbus-codegen
~/.cargo/bin/dbus-codegen-rust <./org.freedesktop.Secrets.xml >mod.rs

echo "done"
