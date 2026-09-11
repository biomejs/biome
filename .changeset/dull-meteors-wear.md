---
"@biomejs/biome": patch
---

Fixed [#7984](https://github.com/biomejs/biome/issues/7984): The fix from [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) now preserves line breaks in multiline conditions with line comments, preventing the right-hand side condition from being commented out.
