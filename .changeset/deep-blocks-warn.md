---
"@biomejs/biome": patch
---

Added the nursery rule [`noExcessiveNestedBlocks`](https://biomejs.dev/linter/rules/no-excessive-nested-blocks/), which reports blocks nested deeper than the configured maximum (4 by default). It checks `if`, `switch`, `try`, and loop statements in JavaScript, and nested rules and at-rules in CSS.
