#!/bin/sh
# cspell:word objdump
# cspell:word readelf
set -e
which dump.sh
llvm-objdump --version
llvm-readelf --version
rustfilt --version
solana --version
sbpf --version
