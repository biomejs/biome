set -eu

cargo run --bin biome -- lint --verbose . 2>&1 | tee /dev/stderr | grep -q file.js
