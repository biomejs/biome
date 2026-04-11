---
"@biomejs/biome": patch
---

Added the nursery lint rule [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/), which prefers `Math.min()` and `Math.max()` over equivalent ternary comparisons.

For example, this code:
```js
const min = a < b ? a : b;
```
is much more readable when rewritten as:
```js
const min = Math.min(a, b);
```
