#!/usr/bin/env bash
set -euo pipefail

TEMP=$(mktemp)
trap "rm -f $TEMP" EXIT

biome() {
	cargo run --bin biome -- "$@"
}

STATUS=0
biome check --write --stdin-file-path="index.html" < "index.html" > "$TEMP"
if ! git diff --no-index "index.html" "$TEMP"; then
	STATUS=1
fi

exit $STATUS
