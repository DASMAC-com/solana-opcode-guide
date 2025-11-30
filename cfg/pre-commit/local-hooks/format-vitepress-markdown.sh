#!/bin/sh
# cspell:word mdformat
# Format markdown with mdformat, then restore VitePress syntax.
# Only fails if there are changes after both operations complete.
set -e

# Create temp directory for checksums.
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

# Store checksums before processing.
for file in "$@"; do
	safe_name=$(echo "$file" | sed 's/[^a-zA-Z0-9]/_/g')
	md5 -q "$file" 2>/dev/null >"$tmpdir/$safe_name.before" || echo "" >"$tmpdir/$safe_name.before"
done

# Run mdformat.
mdformat "$@"

# Restore VitePress syntax that mdformat may have corrupted.
for file in "$@"; do
	# Replace `\<<<` with `<<<`
	sed -i '' 's/\\<<</<<</g' "$file"
	# Replace `\{` with `{`
	sed -i '' 's/\\{/{/g' "$file"
done

# Check if any files actually changed.
changed=0
for file in "$@"; do
	safe_name=$(echo "$file" | sed 's/[^a-zA-Z0-9]/_/g')
	before_checksum=$(cat "$tmpdir/$safe_name.before")
	after_checksum=$(md5 -q "$file" 2>/dev/null || echo "")
	if [ "$before_checksum" != "$after_checksum" ]; then
		changed=1
		break
	fi
done

# Exit with code 1 if files changed.
exit $changed
