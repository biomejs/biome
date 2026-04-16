---
"@biomejs/biome": patch
---

Added the nursery rule [`useReduceTypeParameter`](https://biomejs.dev/linter/rules/use-reduce-type-parameter/). It flags type assertions on the initial value passed to `Array#reduce` and `Array#reduceRight` and recommends using a type parameter instead.

```ts
// before: type assertion on initial value
arr.reduce((sum, num) => sum + num, [] as number[]);

// after: type parameter on the call
arr.reduce<number[]>((sum, num) => sum + num, []);
```
