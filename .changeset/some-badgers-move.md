---
"@biomejs/biome": patch
---

Improved performance for [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles/) by explicitly excluding node_modules from the cycle detection. The performance improvement is directly proportional to how big your dependency tree is.
