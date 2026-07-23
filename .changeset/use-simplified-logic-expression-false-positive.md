---
"@biomejs/biome": patch
---

Fixed [#8577](https://github.com/biomejs/biome/issues/8577): [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) no longer suggests removing right-side boolean literals like `account?.test || false`.
