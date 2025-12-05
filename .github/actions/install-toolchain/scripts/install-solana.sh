#!/bin/sh
set -eu

solana_version="${1}"
install_url="https://release.anza.xyz/${solana_version}/install"

echo "Installing Solana ${solana_version} from ${install_url}"
sh -c "$(curl -sSfL "${install_url}")"
