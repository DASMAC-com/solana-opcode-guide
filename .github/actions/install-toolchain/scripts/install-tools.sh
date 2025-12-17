#!/bin/sh
set -e

SOLANA_VERSION="$1"
PLATFORM_TOOLS_VERSION_DISASSEMBLE="$2"
PLATFORM_TOOLS_VERSION_DUMP="$3"
PLATFORM_TOOLS_VERSION_TEST="$4"
SBPF_REVISION="$5"
RUSTFILT_VERSION="$6"
# Install Solana toolchain.
sh -c "$(curl -sSfL "https://release.anza.xyz/${SOLANA_VERSION}/install")"

# Install platform-tools.
cargo-build-sbf --install-only --tools-version "$PLATFORM_TOOLS_VERSION_DISASSEMBLE"
cargo-build-sbf --install-only --tools-version "$PLATFORM_TOOLS_VERSION_DUMP"
cargo-build-sbf --install-only --tools-version "$PLATFORM_TOOLS_VERSION_TEST"

# Run platform tools SBF install script (cargo-build-sbf skips
# this at install time, so it cache misses at build time).
install.sh

# Install sbpf CLI.
cargo install --git https://github.com/blueshift-gg/sbpf.git --rev "$SBPF_REVISION"

# Install rustfilt.
cargo install rustfilt --version "$RUSTFILT_VERSION"
