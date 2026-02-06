---
"@biomejs/biome": patch
---

Fixed [#8577](https://github.com/biomejs/biome/issues/8577): [`useSimplifiedLogicExpression`](https://biomejs.dev/linter/rules/use-simplified-logic-expression/) no longer suggests simplifying logical expressions when the boolean literal is on the right side. Patterns like `account?.test || false` are intentional boolean coercion — `|| false` ensures the result is always `true` or `false` rather than potentially `undefined`. Left-side simplifications (`true && x`, `false || x`) remain unchanged as they are always safe due to short-circuit semantics.
