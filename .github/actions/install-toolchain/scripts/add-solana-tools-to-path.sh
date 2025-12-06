#!/bin/sh
set -e
SOLANA_RELEASE="$HOME/.local/share/solana/install/active_release/bin"
SBPF_TOOLS="$SOLANA_RELEASE/platform-tools-sdk/sbf"
echo "$SOLANA_RELEASE" >> "$GITHUB_PATH"
echo "$SBPF_TOOLS/scripts" >> "$GITHUB_PATH"
echo "$SBPF_TOOLS/dependencies/platform-tools/llvm/bin" >> "$GITHUB_PATH"
