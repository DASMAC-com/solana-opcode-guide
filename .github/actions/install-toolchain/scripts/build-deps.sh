#!/bin/sh
set -e
SBPF_ARCH_DISASSEMBLE="$1"
TOOLS_VERSION_DISASSEMBLE="$2"
SBPF_ARCH_DUMP="$3"
TOOLS_VERSION_DUMP="$4"
SBPF_ARCH_TEST="$5"
TOOLS_VERSION_TEST="$6"

cd examples
cargo build --package build-deps
cargo test --no-run --package build-deps
cargo clippy --package build-deps --all-targets
(
	cd utils/deps/program &&
		cargo build-sbf --arch "$SBPF_ARCH_DISASSEMBLE" --tools-version "$TOOLS_VERSION_DISASSEMBLE" &&
		cargo build-sbf --arch "$SBPF_ARCH_DUMP" --tools-version "$TOOLS_VERSION_DUMP" &&
		cargo build-sbf --arch "$SBPF_ARCH_TEST" --tools-version "$TOOLS_VERSION_TEST"
)
