#!/usr/bin/env bash
# Bumps the @biomejs/biome wrapper and all platform CLI packages to a single
# version string. Required because the wrapper's optionalDependencies pin
# exact versions — wrapper and platform packages must agree or the wrapper
# refuses to load the patched binary at runtime.
#
# Usage:
#   scripts/skusafe-prepare-release.sh 2.4.14-skusafe.1
set -euo pipefail

VERSION="${1:?usage: skusafe-prepare-release.sh <version>}"

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required" >&2
  exit 1
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WRAPPER="$ROOT/packages/@biomejs/biome/package.json"

# Every @biomejs/cli-* package — the matrix may build only a subset, but we
# keep all of them version-aligned so optionalDependencies stays consistent.
PLATFORM_PKGS=(
  cli-darwin-arm64
  cli-darwin-x64
  cli-linux-arm64
  cli-linux-arm64-musl
  cli-linux-x64
  cli-linux-x64-musl
  cli-win32-arm64
  cli-win32-x64
)

bump() {
  local pkg_json="$1"
  local tmp
  tmp="$(mktemp)"
  jq --arg v "$VERSION" '.version = $v' "$pkg_json" > "$tmp"
  mv "$tmp" "$pkg_json"
}

bump_wrapper_optional_deps() {
  local tmp
  tmp="$(mktemp)"
  jq --arg v "$VERSION" '
    .optionalDependencies |= with_entries(.value = $v)
  ' "$WRAPPER" > "$tmp"
  mv "$tmp" "$WRAPPER"
}

echo "Bumping wrapper to $VERSION"
bump "$WRAPPER"
bump_wrapper_optional_deps

for pkg in "${PLATFORM_PKGS[@]}"; do
  pj="$ROOT/packages/@biomejs/$pkg/package.json"
  if [[ -f "$pj" ]]; then
    echo "Bumping $pkg to $VERSION"
    bump "$pj"
  else
    echo "warn: $pj not found, skipping" >&2
  fi
done

echo "Done. Version is now $VERSION across wrapper + platform packages."
