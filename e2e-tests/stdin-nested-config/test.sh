#!/usr/bin/env bash
set -euo pipefail

TEMP=$(mktemp)
trap 'rm -f $TEMP' EXIT

biome() {
    cargo run --bin biome -- "$@"
}

STATUS=0
diff_stdin_filepath() {
    biome format --stdin-file-path="$1" < "$1" > "$TEMP"
    if ! git diff --no-index "$1.formatted" "$TEMP"; then
        STATUS=1
    fi
}

diff_stdin_filepath app.js
diff_stdin_filepath subdirectory/lib.js
diff_stdin_filepath subdirectory/typed.ts

biome format --stdin-file-path="donotformat.ts" < "donotformat.ts" > "$TEMP"
if ! git diff --no-index "donotformat.ts" "$TEMP"; then
    STATUS=1
fi

exit $STATUS
