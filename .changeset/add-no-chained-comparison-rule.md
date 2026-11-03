---
"@biomejs/biome": patch
---

Added the new nursery rule [`noChainedComparison`](https://biomejs.dev/linter/rules/no-chained-comparison/), which reports comparisons chained like `a < b < c` or `a === b === c`. JavaScript compares `a` with `b` first, and then compares the resulting `true` or `false` with `c`, so `3 > 2 > 1` is `false`.

```js
if (a < b < c) {}
```
