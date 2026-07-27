---
"@biomejs/biome": patch
---

Fixed [#11092](https://github.com/biomejs/biome/issues/11092): the [`noUselessTernary`](https://biomejs.dev/linter/rules/no-useless-ternary/) fix no longer doubles the whitespace around the operator when the ternary test is a binary, `instanceof`, or `in` expression, and no longer drops the whitespace when it inverts a comparison.
