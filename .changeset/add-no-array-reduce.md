---
"@biomejs/biome": patch
---

Added the nursery rule [`noArrayReduce`](https://biomejs.dev/linter/rules/no-array-reduce/), which disallows the `reduce()` and `reduceRight()` array methods in favor of loops and methods such as `map()` and `filter()`. Calls whose function only returns a single arithmetic, comparison, or bitwise operation, such as `(total, item) => total + item`, are allowed.

```js
const counts = words.reduce((counts, word) => {
    counts[word] = (counts[word] ?? 0) + 1;
    return counts;
}, {});
```
