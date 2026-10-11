---
"@biomejs/biome": patch
---

Added the nursery rule [`noDoubleComparison`](https://biomejs.dev/linter/rules/no-double-comparison/), which reports two comparisons of the same values that can be combined into one. For example, `x === y || x < y` can be written as `x <= y`.
