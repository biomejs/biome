---
"@biomejs/biome": patch
---

[`noUnsafePlusOperands`](https://biomejs.dev/linter/rules/no-unsafe-plus-operands/) now detects mixed number and bigint additions when an operand type comes from a generic type alias.

```ts
type Identity<T> = T;
declare const big: Identity<bigint>;
declare const num: number;
big + num;
```
