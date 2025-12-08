#!/bin/sh
set -e
SBPF_ARCH_A="$1"
TOOLS_VERSION_A="$2"
SBPF_ARCH_B="$3"
TOOLS_VERSION_B="$4"

cd examples
cargo build --package build-deps
cargo test --no-run --package build-deps
(
	cd utils/deps/program &&
		cargo build-sbf --arch "$SBPF_ARCH_A" --tools-version "$TOOLS_VERSION_A" &&
		cargo build-sbf --arch "$SBPF_ARCH_B" --tools-version "$TOOLS_VERSION_B"
)
