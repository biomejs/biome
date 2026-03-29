---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUnsafePlusOperands`](https://biomejs.dev/linter/rules/no-unsafe-plus-operands/), which reports `+` and `+=` operations that use object-like, `symbol`, `unknown`, or `never` operands, or that mix `number` with `bigint`.
