#!/bin/sh

set -eu

for x in *; do
    if test -d "$x"; then
        echo "Testing $x..."
        cd "$x"
        sh test.sh
        cd ..
    fi
done
