#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

if ! command -v wasm-pack &> /dev/null; then
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

if [ "$1" == "preview" ]; then
  pnpm build:wasm
else
  pnpm build:wasm
fi

# Run pnpm install again to ensure wasm gets linked
pnpm install

# Finally build website
pnpm build:js
