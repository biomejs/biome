#!/usr/bin/env bash
# Run cargo clippy on any crate touched by a Write, Edit, or MultiEdit call.
set -euo pipefail

input=$(cat)

# Extract all edited .rs file paths, resolve to crate roots, deduplicate, run clippy.
echo "$input" | jq -r '
  [
    (.tool_input.file_path // empty),
    (.tool_input.edits[]?.file_path // empty)
  ][] | select(endswith(".rs"))
' | while IFS= read -r file; do
  dir=$(dirname "$file")
  while [[ "$dir" != "/" && ! -f "$dir/Cargo.toml" ]]; do
    dir=$(dirname "$dir")
  done
  [[ -f "$dir/Cargo.toml" ]] && echo "$dir"
done | sort -u | while IFS= read -r dir; do
  pkg=$(grep -m1 '^name' "$dir/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
  echo "clippy: $pkg"
  cargo clippy -p "$pkg" 2>&1 | head -50
done
