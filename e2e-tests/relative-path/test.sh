set -eu

cargo run --bin biome -- lint --verbose . 2>&1 | grep -q file.js
