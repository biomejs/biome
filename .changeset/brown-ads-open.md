---
"@biomejs/biome": patch
---

Added the recommended nursery rule [`noOctal`](https://biomejs.dev/linter/rules/no-octal/), which disallows legacy octal literals such as `071` and decimal literals with leading zeros such as `08`. Use `0o71` for explicit octal notation.
