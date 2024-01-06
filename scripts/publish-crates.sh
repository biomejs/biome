
#!/usr/bin/env bash
# -e -o pipefail: Fail on when happening command errors
set -eo pipefail

# Usage:
#   ./scripts/publish-crates.sh [--execute]
# Options:
#   --execute: Execute the publish command, otherwise dry-run (default)

# publish crates (order is important)
published_crates=(
  "biome_diagnostics_categories"
  "biome_diagnostics_macros"
  "biome_unicode_table"
  "biome_markup"
  "biome_text_size"
  "biome_console"
  "biome_text_edit"
  "biome_rowan"
  "biome_aria_metadata"
  "biome_aria"
  "biome_diagnostics"
  "biome_analyze"
  "biome_fs"
  "biome_parser"
  "biome_control_flow"
  "biome_suppression"
  "biome_js_syntax"
  "biome_js_factory"
  "biome_js_parser"
  "biome_json_syntax"
  "biome_json_factory"
  "biome_json_parser"
  "biome_css_syntax"
  "biome_css_factory"
  "biome_css_parser"
  "biome_deserialize"
  "biome_formatter"
  "biome_js_semantic"
  "biome_js_analyze"
  "biome_js_formatter"
  "biome_js_transform"
  "biome_json_analyze"
  "biome_json_formatter"
  "biome_css_formatter"
)

for crate in "${published_crates[@]}"; do
  echo "============================================"
  echo "Publishing crate: $crate"
  echo "============================================"
  if [ "$1" = "--execute" ]; then
    cargo publish -p $crate
  else
    cargo publish --dry-run --no-verify --allow-dirty -p $crate
  fi
done
