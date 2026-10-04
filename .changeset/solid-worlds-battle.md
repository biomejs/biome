---
"@biomejs/biome": patch
---

Added the nursery rule [`noProcessExit`](https://biomejs.dev/linter/rules/no-process-exit/), which disallows calling `process.exit()`. For example, `process.exit(1);` is now reported.
