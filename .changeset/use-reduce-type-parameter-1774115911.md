---
"@biomejs/biome": patch
---

Added the nursery rule [`useReduceTypeParameter`](https://biomejs.dev/linter/rules/use-reduce-type-parameter/). It flags type assertions on the initial value passed to `Array#reduce` and `Array#reduceRight` and recommends using a type parameter instead. The autofix removes the assertion and applies that type as a generic argument.
