set -u

# `head` exits after the first line, so Biome keeps writing to a closed pipe.
# It should discard the output and exit with 1 because of the lint errors,
# instead of panicking (101) or aborting (134).
cargo run -q --bin biome -- check file.js 2>&1 | head -1 > /dev/null
[ "${PIPESTATUS[0]}" -eq 1 ]
