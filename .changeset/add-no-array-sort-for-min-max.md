---
"@biomejs/biome": patch
---

Added the new nursery rule [`noArraySortForMinMax`](https://biomejs.dev/linter/rules/no-array-sort-for-min-max/), which reports sorting an array only to read its smallest or largest value. Use `Math.min()` or `Math.max()` instead.

```js
const minimum = array.toSorted((a, b) => a - b)[0];
```
