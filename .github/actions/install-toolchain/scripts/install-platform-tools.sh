#!/bin/sh
set -e
cargo-build-sbf --tools-version "$1" --install-only
