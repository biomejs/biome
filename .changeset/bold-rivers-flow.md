---
"@biomejs/biome": patch
---

[`useArraySortCompare`](https://biomejs.dev/linter/rules/use-array-sort-compare/) now detects compare-less `sort()` on arrays typed through a generic type alias.

```ts
type Id<T> = T;
declare const xs: Id<number[]>;
xs.sort();
```
