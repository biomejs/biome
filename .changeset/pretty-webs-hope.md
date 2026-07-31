---
"@biomejs/biome": minor
---

Added Android (Termux) support to the npm distribution ([#1340](https://github.com/biomejs/biome/issues/1340)). `npm install @biomejs/biome` now works on Termux through the new `@biomejs/cli-android-arm64` and `@biomejs/cli-android-x64` packages, which reuse the static musl binaries.
