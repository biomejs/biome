---
"@biomejs/biome": patch
---

Fixed [#11351](https://github.com/biomejs/biome/issues/11351): [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) no longer reports `x || false` and `x && true`, because rewriting them to `x` changes the result when `x` is not a boolean. Biome still reports `false || x` and `true && x`.
