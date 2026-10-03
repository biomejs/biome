---
"@biomejs/biome": patch
---

Added the nursery rule [`noExcessiveNestedBlocks`](https://biomejs.dev/linter/rules/no-excessive-nested-blocks/), which reports `if`, `switch`, `try`, and loop statements nested deeper than the configured maximum (4 by default).
