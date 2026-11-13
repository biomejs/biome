---
"@biomejs/biome": patch
---

Fixed [`noUnsafePlusOperands`](https://biomejs.dev/linter/rules/no-unsafe-plus-operands/) treating the negation of a `bigint` value as a `number`. Code such as `-value + 1n`, where `value` is declared as `bigint`, is no longer reported for mixing `number` and `bigint`.
