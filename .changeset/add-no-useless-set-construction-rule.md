---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessSetConstruction`](https://biomejs.dev/linter/rules/no-useless-set-construction/), which reports a `new Set()` that only copies a value used with a `Set` method such as `union()` or `isSubsetOf()`. These methods accept a `Set` or a `Map` directly, don't change the `Set` they are called on, and return a new `Set` when they combine values.

```js
const selected = new Set([1, 2]);
const other = new Set([2, 3]);

selected.union(new Set(other));
new Set(selected).union(other);
new Set(selected.union(other));
```
