#!/bin/sh
set -eu

llvm_package="${1}"
llvm_path="/usr/lib/${llvm_package}"

if [ -d "${llvm_path}" ]; then
	echo "LLVM installation found at ${llvm_path}"
else
	echo "Error: ${llvm_path} not found"
	exit 1
fi
