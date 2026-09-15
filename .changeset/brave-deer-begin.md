---
"@biomejs/biome": patch
---

Fixed [#11351](https://github.com/biomejs/biome/issues/11351): [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) no longer reports boolean literals on the right side of `||` and `&&`, such as `x || false`, because removing them can change the result of the expression.
