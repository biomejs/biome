set -u

# Build separately instead of using `cargo run`, so that output from Cargo
# itself (such as manifest warnings) doesn't go into the pipe. Otherwise Cargo
# writes to the closed pipe and its exit code is checked instead of Biome's.
cargo build -q --bin biome || exit 1

# `head` exits after the first line, so Biome keeps writing to a closed pipe.
# It should discard the output and exit with 1 because of the lint errors,
# instead of panicking (101) or aborting (134).
"${CARGO_TARGET_DIR:-../../target}/debug/biome" check file.js 2>&1 | head -1 > /dev/null
[ "${PIPESTATUS[0]}" -eq 1 ]
