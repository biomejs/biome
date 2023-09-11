#!/bin/bash

# https://stackoverflow.com/a/246128/3549270
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"

if ! cargo fuzz --help >&/dev/null; then
  cargo install --git https://github.com/rust-fuzz/cargo +stable-fuzz.git
fi

if [ ! -d corpus/rome_format_all ]; then
  mkdir -p corpus/rome_format_all
  cd corpus/rome_format_all
  if [ -z ${CI+x} ]; then
    read -p "Would you like to build a corpus from a javascript source code dataset? (this will take a long time!) [Y/n] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      curl -L http://files.srl.inf.ethz.ch/data/js_dataset.tar.gz | tar xzO data.tar.gz | tar xz
      find . -type d -exec chmod 755 {} \;
      find . -type f -exec chmod 644 {} \;
    fi
  fi
  cp -r "../../../crates/biome_js_parser/test_data" .
  find . -name \*.rast -delete
  cd -
  cargo fuzz cmin --strip-dead-code --features rome_all -s none rome_format_all
fi

if [ ! -d corpus/rome_format_json ]; then
  mkdir -p corpus/rome_format_json
  cd corpus/rome_format_json
  cp -r "../../../crates/biome_json_parser/tests/json_test_suite" .
  find . -name \*.rast -delete
  cd -
  cargo fuzz cmin --strip-dead-code -s none rome_format_json
fi

echo "Done! You are ready to fuzz."
