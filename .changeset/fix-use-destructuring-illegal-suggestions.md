---
"@biomejs/biome": patch
---

Fixed [#8480](https://github.com/biomejs/biome/issues/8480): [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring/) no longer suggests illegal object destructuring for bare assignment expressions by default. Added `variableDeclarator` and `assignmentExpression` options to control which contexts enforce destructuring, matching ESLint's `prefer-destructuring` configuration. When assignment checking is enabled, the diagnostic for object destructuring now instructs users to wrap the assignment in parentheses.
