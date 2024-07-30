#!/bin/sh
set -eu

CURRENT_DIR=$(dirname "$0")
IS_X_CALL=false
CACHE_DIR="."

BINARY_NAME='biome'
BINARY_NAME_SCOPE='@biomejs/cli-'

# Test if calling via `npx` or `bunx`
if "$(echo pwd)" | grep -q ".bun/install/cache"; then
  IS_X_CALL=true
  CACHE_DIR=$(bun pm cache ls)
elif "$(echo pwd)" | grep -q ".npm/_npx"; then
  IS_X_CALL=true
  CACHE_DIR=$(dirname "$(pwd)")
fi

# Search and find binary
if $IS_X_CALL; then
  BIOME_BIN=$(find "${CACHE_DIR}" -iname "${BINARY_NAME}" | grep -s "${BINARY_NAME_SCOPE}" || echo "")
else
  BIOME_BIN=$(find . -iname "${BINARY_NAME}" | grep -s "${BINARY_NAME_SCOPE}" || echo "")
fi

# Check node_modules
if [ -z "${BIOME_BIN}" ]; then
  echo "\`node_modules\` was not installed"
  exit 1
fi

# Trim variables after success checks
BIOME_BIN=$(realpath -q "${BIOME_BIN}")

# Make it executable
if test -f "${BIOME_BIN}"; then
  chmod +x "${BIOME_BIN}"
fi

# Replace binary in `bin` field for later use
if test -f "${CURRENT_DIR}/package.json"; then
  sed -i.bak "s|bin.sh|${BIOME_BIN}|g" "${CURRENT_DIR}/package.json"
  rm -rf "package.json.bak"
elif echo "${CURRENT_DIR}" | grep -q ".bin"; then
  ln -sf "${BIOME_BIN}" "$0"
fi

# Run currently until next run
"${BIOME_BIN}" "$@"
