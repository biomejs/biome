---
"@biomejs/biome": patch
---

Fixed [#11351](https://github.com/biomejs/biome/issues/11351): [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) no longer reports `x || false` and `x && true`. Rewriting them to `x` changes the result when `x` is not a boolean (for example, `0 || false` is `false`, not `0`). The safe cases `false || x` and `true && x` are still reported.
