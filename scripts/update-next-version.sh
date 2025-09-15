#!/usr/bin/env bash
set -euo pipefail

# Script: update_next_version.sh
# Purpose:
#   - Search for occurrences of: version: "next"
#     in Rust source files under: crates/biome_*_analyze
#   - Optionally replace "next" with a provided new string (using sed).
#
# Requirements:
#   - ripgrep (rg)
#   - sed (GNU sed recommended; script targets Linux)
#
# Usage:
#   Just search:
#     ./update_next_version.sh
#
#   Replace "next" with "1.2.3":
#     ./update_next_version.sh --replace 1.2.3
#
# Options:
#   -r, --replace <STRING>   Replace 'version: "next"' with 'version: "<STRING>"'
#   -n, --dry-run            Show which files would change (with --replace), but don't modify
#   -q, --quiet              Suppress normal search output (still prints summary)
#   -h, --help               Show help
#
# Exit codes:
#   0 success (even if no matches)
#   1 usage / argument errors

PATTERN='version:[[:space:]]*"next"'
SEARCH_ROOT='crates/biome_*_analyze'

print_help() {
  cat <<'EOF'
Search (and optionally replace) occurrences of: version: "next"
within: crates/biome_*_analyze/**/*.rs

Usage:
  update_next_version.sh               # list matches
  update_next_version.sh -r 1.2.3      # replace next -> 1.2.3
  update_next_version.sh -r 1.2.3 -n   # dry-run replacement

Options:
  -r, --replace <STRING>  New version string to substitute for "next"
  -n, --dry-run           Show files that would change (with --replace) but don't edit
  -q, --quiet             Suppress match lines (still shows summary)
  -h, --help              Show this help

Requires: ripgrep (rg), sed
EOF
}

need_rg() {
  if ! command -v rg >/dev/null 2>&1; then
    echo "Error: ripgrep (rg) is required but not installed." >&2
    exit 1
  fi
}

need_sed() {
  if ! command -v sed >/dev/null 2>&1; then
    echo "Error: sed is required but not installed." >&2
    exit 1
  fi
}

REPLACE_VALUE=""
DRY_RUN="false"
QUIET="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    -r|--replace)
      if [[ $# -lt 2 ]]; then
        echo "Error: --replace requires an argument." >&2
        exit 1
      fi
      REPLACE_VALUE="$2"
      shift 2
      ;;
    -n|--dry-run)
      DRY_RUN="true"
      shift
      ;;
    -q|--quiet)
      QUIET="true"
      shift
      ;;
    -h|--help)
      print_help
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      echo "Use --help for usage." >&2
      exit 1
      ;;
  esac
done

need_rg

# Collect matching files
mapfile -t MATCH_FILES < <(rg -l "$PATTERN" $SEARCH_ROOT --type rust 2>/dev/null || true)
TOTAL_FILES=${#MATCH_FILES[@]}

if [[ -z "$REPLACE_VALUE" ]]; then
  # Search-only mode
  if [[ $TOTAL_FILES -eq 0 ]]; then
    echo "No occurrences of version: \"next\" found under $SEARCH_ROOT"
    exit 0
  fi

  if [[ "$QUIET" != "true" ]]; then
    rg "$PATTERN" $SEARCH_ROOT --type rust --line-number
    echo
  fi

  echo "Found $TOTAL_FILES file(s) containing version: \"next\"."
  if [[ $TOTAL_FILES -gt 0 ]]; then
  	echo "These rules need to be updated before release!"
  	echo "To replace 'next' with another value, rerun with --replace <STRING>."
	fi
  exit 0
fi

# Replacement mode
if [[ "$REPLACE_VALUE" == "next" ]]; then
  echo "Replacement value is already 'next'; nothing to do."
  exit 0
fi

if [[ $TOTAL_FILES -eq 0 ]]; then
  echo "No occurrences found to replace."
  exit 0
fi

need_sed

# Escape replacement for sed (&, |, /, \)
escape_replacement() {
  printf '%s' "$1" | sed -e 's/[&/|\\]/\\&/g'
}

ESCAPED_REPLACE=$(escape_replacement "$REPLACE_VALUE")

echo "Will replace: version: \"next\"  -->  version: \"$REPLACE_VALUE\""
echo "Affected files ($TOTAL_FILES):"
for f in "${MATCH_FILES[@]}"; do
  echo "  $f"
done

if [[ "$DRY_RUN" == "true" ]]; then
  echo
  echo "Dry-run enabled: no files modified."
  exit 0
fi

# Perform in-place substitutions
for f in "${MATCH_FILES[@]}"; do
  # Using | as delimiter to avoid escaping / in paths
  sed -E -i "s|version:[[:space:]]*\"next\"|version: \"$ESCAPED_REPLACE\"|g" "$f"
done

echo
echo "Replacement complete. Modified $TOTAL_FILES file(s)."
echo "Review changes with: git diff"
