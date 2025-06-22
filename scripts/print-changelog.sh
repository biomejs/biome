#!/bin/bash

# Script to extract changelog section for a specific version
# Usage: ./print-changelog.sh [version] [changelog-file]

# Default values
VERSION=""
CHANGELOG_FILE="packages/@biomejs/biome/CHANGELOG.md"

# Parse arguments
if [ $# -eq 0 ]; then
    exit 1
fi

VERSION="$1"
if [ $# -eq 2 ]; then
    CHANGELOG_FILE="$2"
fi

# Check if changelog file exists
if [ ! -f "$CHANGELOG_FILE" ]; then
    exit 1
fi

# Extract the section for the specified version
awk -v version="$VERSION" '
BEGIN {
    found = 0
    printing = 0
}

# Found the version header
/^## / {
    if ($2 == version) {
        found = 1
        printing = 1
        print
        next
    } else if (printing) {
        # Found next version section, stop printing
        exit
    }
}

# Print lines when we are in the target version section
printing {
    print
}

END {
    if (!found) {
        exit 1
    }
}
' "$CHANGELOG_FILE"
