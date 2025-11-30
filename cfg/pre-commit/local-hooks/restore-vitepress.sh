#!/bin/bash
# Restore VitePress syntax corrupted by markdown formatters.
set -e
for file in "$@"; do
	# Replace `\<<<` with `<<<`
	sed -i '' 's/\\<<</<<</g' "$file"
	# Replace `\{` with `{`
	sed -i '' 's/\\{/{/g' "$file"
done
