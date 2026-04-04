---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessTypeConversion`](https://biomejs.dev/linter/rules/no-useless-type-conversion/), which reports redundant primitive conversion patterns such as `String(value)` when `value` is already a string.
