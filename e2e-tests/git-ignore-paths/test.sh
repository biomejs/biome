set +eu

if cargo run --bin biome -- lint .; then
  # Command succeeded
  exit 1
else
  # Command failed
  exit 0
fi

