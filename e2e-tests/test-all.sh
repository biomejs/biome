#!/usr/bin/env bash

set -eu

cd "$(dirname "$0")"
if [[ -z "${1:-}" ]]; then FILTER="*"; else FILTER="*$1*"; fi

for x in *; do
    if test -d "$x"; then
			if [[ "$x" != $FILTER ]]; then
				echo "Skipping $x"
				continue
			fi
        echo "Testing $x..."
        pushd "$x"
        bash test.sh
        popd
    fi
done
