---
"@biomejs/biome": patch
---

Fixed [#11092](https://github.com/biomejs/biome/issues/11092): The [`noUselessTernary`](https://biomejs.dev/linter/rules/no-useless-ternary/) quick fix no longer produces a double space before the operator when simplifying ternary expressions like `x > -1 ? true : false`.
