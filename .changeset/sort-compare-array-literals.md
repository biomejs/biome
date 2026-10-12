---
"@biomejs/biome": patch
---

Fixed [#12247](https://github.com/biomejs/biome/issues/12247): type inference now infers array literals without `as const` as arrays, as TypeScript does. For example, `[1, "a"]` is inferred as `(string | number)[]` instead of a tuple. As a result, [`useArraySortCompare`](https://biomejs.dev/linter/rules/use-array-sort-compare/) now reports `sort()` and `toSorted()` calls on arrays without a type annotation.

```ts
const numbers = [1, 2, 3, 10, 20, 30];
numbers.sort();
```
