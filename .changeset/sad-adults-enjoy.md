---
"@biomejs/biome": patch
---

Added the nursery rule [`useArraySortCompare`](https://biomejs.dev/linter/rules/use-array-sort-compare/). Require Array#sort and Array#toSorted calls to always provide a compareFunction.

**Invalid:**
```js
const array = [];
array.sort();
```

**Valid:**
```js
const array = [];
array.sort((a, b) => a - b);
```
