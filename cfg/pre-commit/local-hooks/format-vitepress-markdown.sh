#!/bin/sh
# cspell:word mdformat
# Format markdown with mdformat, then restore VitePress syntax.
# Only fails if there are changes after both operations complete.
set -e

# Create temp directory for checksums.
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

# Cross-platform checksum command.
get_checksum() {
	if command -v md5 >/dev/null 2>&1; then
		md5 -q "$1" 2>/dev/null || echo ""
	else
		md5sum "$1" 2>/dev/null | cut -d' ' -f1 || echo ""
	fi
}

# Store checksums before processing.
for file in "$@"; do
	safe_name=$(echo "$file" | sed 's/[^a-zA-Z0-9]/_/g')
	get_checksum "$file" >"$tmpdir/$safe_name.before"
done

# Run mdformat.
mdformat "$@"

# Restore VitePress syntax that mdformat may have corrupted.
for file in "$@"; do
	# Replace `\<<<` with `<<<` and `\{` with `{`
	sed 's/\\<<</<<</g; s/\\{/{/g' "$file" >"$file.tmp" && mv "$file.tmp" "$file"
done

# Check if any files actually changed.
changed=0
for file in "$@"; do
	safe_name=$(echo "$file" | sed 's/[^a-zA-Z0-9]/_/g')
	before_checksum=$(cat "$tmpdir/$safe_name.before")
	after_checksum=$(get_checksum "$file")
	if [ "$before_checksum" != "$after_checksum" ]; then
		changed=1
		break
	fi
done

# Exit with code 1 if files changed.
exit $changed
