#!/bin/bash
# Restore VitePress syntax corrupted by markdown formatters.
set -e
for file in "$@"; do
  if [[ "$file" == docs/src/* ]] && [[ "$file" == *.md ]]; then
    # Replace `\<<<` with `<<<`
    sed -i '' 's/\\<<</<<</g' "$file"
    # Replace `\{` with `{`
    sed -i '' 's/\\{/{/g' "$file"
  fi
done
