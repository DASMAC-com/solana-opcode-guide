#!/bin/sh
set -e

SOLANA_VERSION="$1"
PLATFORM_TOOLS_VERSION="$2"
SBPF_REVISION="$3"

# Install Solana toolchain.
sh -c "$(curl -sSfL "https://release.anza.xyz/${SOLANA_VERSION}/install")"

# Install platform-tools.
cargo-build-sbf --install-only --tools-version "$PLATFORM_TOOLS_VERSION"

# Run platform tools SBF install script (cargo-build-sbf skips
# this at install time, so it cache misses at build time).
install.sh

# Install sbpf CLI.
cargo install --git https://github.com/blueshift-gg/sbpf.git --rev "$SBPF_REVISION"
