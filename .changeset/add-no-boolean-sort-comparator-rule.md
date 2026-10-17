---
"@biomejs/biome": patch
---

Added the nursery rule [`noBooleanSortComparator`](https://biomejs.dev/linter/rules/no-boolean-sort-comparator/), which reports sort comparators that return a boolean instead of a number. For example, `array.sort((a, b) => a > b)` never puts the first item before the second, so the array isn't sorted correctly.
