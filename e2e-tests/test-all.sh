#!/usr/bin/env bash

# Usage:
#  ./test-all.sh
#  ./test-all.sh stdin   # to filter tests by name

set -eu

# Change to the script's directory
cd "$(dirname "$0")"

# Glob matcher
if [[ -z "${1:-}" ]]; then FILTER="*"; else FILTER="*$1*"; fi

redecho() {
    echo -e "\033[1;31m$1\033[0m"
}

bail() {
    redecho "Error: $1"
    exit 1
}

for x in *; do
    if test -d "$x"; then
        if [[ "$x" != $FILTER ]]; then
            echo "Skipping $x"
            continue
        fi
        echo "Testing $x..."
        pushd "$x" > /dev/null
        bash test.sh || bail "Test failed: $x. To re-run: $0 $x"
        popd > /dev/null
    fi
done
