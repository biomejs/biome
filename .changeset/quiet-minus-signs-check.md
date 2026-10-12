---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUnsafeUnaryMinus`](https://biomejs.dev/linter/rules/no-unsafe-unary-minus/), which uses type information to report unary `-` applied to values that aren't a `number` or `bigint`.

```ts
declare const value: string;
-value;
```
