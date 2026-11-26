---
"@biomejs/biome": patch
---

Added the nursery rule [`useSetMethods`](https://biomejs.dev/linter/rules/use-set-methods/), which reports code that combines or compares `Set`s by copying them into arrays, and suggests the matching `Set` method, such as `union()`, `intersection()`, `difference()`, `isSubsetOf()`, or `isDisjointFrom()`.

```js
const a = new Set([1, 2]);
const b = new Set([2, 3]);

new Set([...a, ...b]);
[...a].filter((value) => b.has(value));
[...a].every((value) => b.has(value));
a.intersection(b).size === 0;
```
