---
"@biomejs/biome": patch
---

Fixed [#11351](https://github.com/biomejs/biome/issues/11351): [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) now correctly marks the fix for `expr || false` and `expr && true` as unsafe. Previously the fix was marked safe, but it can change runtime behavior when the non-literal operand is not strictly boolean (e.g. `boolean | undefined`).
