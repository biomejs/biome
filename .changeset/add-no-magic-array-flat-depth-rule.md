---
"@biomejs/biome": patch
---

Added the new nursery rule [`noMagicArrayFlatDepth`](https://biomejs.dev/linter/rules/no-magic-array-flat-depth/), which reports number literals other than `1` passed as the depth of `Array.prototype.flat()` unless a comment inside the parentheses explains them.

```js
const items = array.flat(2);
```
