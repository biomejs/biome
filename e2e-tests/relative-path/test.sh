set -eu

cargo run --bin biome -- lint . 2>&1 | grep -q debugger
