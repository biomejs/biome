---
"@biomejs/biome": patch
---

Added the nursery rule [`noNewRequire`](https://biomejs.dev/linter/rules/no-new-require/), which disallows calling `require` with `new`, such as `new require("app-header")`.
