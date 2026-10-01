---
"@biomejs/biome": patch
---

Added the nursery rule [`noExcessiveStatementsPerFunction`](https://biomejs.dev/linter/rules/no-excessive-statements-per-function/), a port of ESLint's [`max-statements`](https://eslint.org/docs/latest/rules/max-statements). It reports functions that contain more statements than the configured `max` (default: `10`).
