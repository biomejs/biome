#!/bin/bash
set -eu

# Print a changelog section (default: first section).

VERSION=''

if test -n "${1:-}" && grep -Eq "^## $1($| )" CHANGELOG.md; then
    # The specified version has a dedicated section in the changelog
    VERSION="$1"
fi

# print Changelog of $VERSION
awk -v version="$VERSION" '/^## / { if (p) { exit }; if (version == "" || $2 == version) { p=1; next} } p' CHANGELOG.md
