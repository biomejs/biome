#!/bin/bash

# https://stackoverflow.com/a/246128/3549270
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"

mkdir -p corpus/biome_format_all
cd corpus/biome_format_all
cp -r "../../../crates/biome_js_parser/test_data" .
find . -name \*.rast -delete
cd -
cargo fuzz cmin --strip-dead-code --features biome_all -s none biome_format_all

mkdir -p corpus/biome_format_json
cd corpus/biome_format_json
cp -r "../../../crates/biome_json_parser/tests/json_test_suite" .
find . -name \*.rast -delete
cd -
cargo fuzz cmin --strip-dead-code -s none biome_format_json

echo "Done! You are ready to fuzz."
