#!/usr/bin/env bash
set -euo pipefail

# Script: update-commonmark-spec.sh
# Purpose:
#   Download a specific version of the CommonMark spec.json and update
#   both the spec file and the provenance comment in commonmark.rs.
#
# Usage:
#   ./scripts/update-commonmark-spec.sh <version>
#
# Example:
#   ./scripts/update-commonmark-spec.sh 0.31.2
#
# After running, verify with:
#   just test-markdown-conformance

SPEC_PATH="xtask/coverage/src/markdown/spec.json"
RS_PATH="xtask/coverage/src/markdown/commonmark.rs"

print_help() {
  cat <<'EOF'
Download a specific version of the CommonMark spec.json and update provenance.

Usage:
  update-commonmark-spec.sh <version>

Example:
  update-commonmark-spec.sh 0.31.2

This will:
  1. Download https://spec.commonmark.org/<version>/spec.json
  2. Replace xtask/coverage/src/markdown/spec.json
  3. Update the provenance comment in commonmark.rs

After updating, verify with:
  just test-markdown-conformance
EOF
}

if [[ $# -lt 1 ]]; then
  print_help
  exit 1
fi

if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
  print_help
  exit 0
fi

VERSION="$1"
URL="https://spec.commonmark.org/${VERSION}/spec.json"
TODAY=$(date +%Y-%m-%d)

echo "Downloading CommonMark spec version ${VERSION}..."
echo "URL: ${URL}"

if ! curl -fsSL "${URL}" -o "${SPEC_PATH}.tmp"; then
  echo "Error: Failed to download spec from ${URL}" >&2
  echo "Check that the version exists at https://spec.commonmark.org/" >&2
  rm -f "${SPEC_PATH}.tmp"
  exit 1
fi

mv "${SPEC_PATH}.tmp" "${SPEC_PATH}"

# Count examples
if command -v jq >/dev/null 2>&1; then
  COUNT=$(jq 'length' "${SPEC_PATH}")
else
  echo "Warning: jq not found, cannot count examples" >&2
  COUNT="unknown"
fi

# Update provenance comment in commonmark.rs
echo "Updating provenance in ${RS_PATH}..."

sed -i.bak \
  -e "s|//   Version: .*|//   Version: ${VERSION}|" \
  -e "s|//   URL: .*|//   URL: ${URL}|" \
  -e "s|//   Downloaded: .*|//   Downloaded: ${TODAY}|" \
  -e "s|//   Examples: .*|//   Examples: ${COUNT}|" \
  "${RS_PATH}"

rm -f "${RS_PATH}.bak"

# Print summary
echo
echo "Updated:"
echo "  - ${SPEC_PATH}"
echo "  - ${RS_PATH}"
echo
echo "Provenance:"
echo "  Version: ${VERSION}"
echo "  URL: ${URL}"
echo "  Downloaded: ${TODAY}"
echo "  Examples: ${COUNT}"
echo
echo "Next step:"
echo "  just test-markdown-conformance"
