---
"@biomejs/biome": patch
---

Fixed [#12523](https://github.com/biomejs/biome/issues/12523): [`noUnsafeUnaryMinus`](https://biomejs.dev/linter/rules/no-unsafe-unary-minus/) no longer reports a parameter that has a default value. Inside the function body, the default replaces `undefined`, so `bleed` below is a `number`.

```ts
(bleed: number | undefined = 0) => -bleed;
```
