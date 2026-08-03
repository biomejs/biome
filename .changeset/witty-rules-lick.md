---
"@biomejs/biome": patch
---

Improved the performance of [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles/) by skipping graph traversals for imports that cannot be part of a cycle.
