#!/bin/sh
set -e

# Set platform identifier.
PLATFORM="${RUNNER_OS}-${RUNNER_ARCH}"
echo "PLATFORM=${PLATFORM}" >> "$GITHUB_ENV"
echo "platform=${PLATFORM}" >> "$GITHUB_OUTPUT"

# Add Solana tools to PATH.
SOLANA_RELEASE="$HOME/.local/share/solana/install/active_release/bin"
SBPF_TOOLS="$SOLANA_RELEASE/platform-tools-sdk/sbf"
echo "$SOLANA_RELEASE" >> "$GITHUB_PATH"
echo "$SBPF_TOOLS/scripts" >> "$GITHUB_PATH"
echo "$SBPF_TOOLS/dependencies/platform-tools/llvm/bin" >> "$GITHUB_PATH"
