---
"@biomejs/biome": patch
---

Added the nursery rule [`useReduceTypeParameter`](https://biomejs.dev/linter/rules/use-reduce-type-parameter/). This rule enforces using a type parameter on `Array#reduce` and `Array#reduceRight` instead of a type assertion (`as` or angle bracket) on the initial value.

```ts
// Before:
arr.reduce((acc, x) => acc.concat(x), [] as number[]);

// After (with autofix):
arr.reduce<number[]>((acc, x) => acc.concat(x), []);
```
