#!/bin/sh
set -e
SBPF_ARCH_DUMP="$1"
TOOLS_VERSION_DUMP="$2"
SBPF_ARCH_TEST="$3"
TOOLS_VERSION_TEST="$4"

cd examples
cargo build --package build-deps
cargo test --no-run --package build-deps
(
	cd utils/deps/program &&
		cargo build-sbf --arch "$SBPF_ARCH_DUMP" --tools-version "$TOOLS_VERSION_DUMP" &&
		cargo build-sbf --arch "$SBPF_ARCH_TEST" --tools-version "$TOOLS_VERSION_TEST"
)
