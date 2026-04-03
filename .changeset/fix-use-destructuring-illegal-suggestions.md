---
"@biomejs/biome": patch
---

Fixed [#8480](https://github.com/biomejs/biome/issues/8480): [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring/) now provides `variableDeclarator` and `assignmentExpression` options to control which contexts enforce destructuring, matching ESLint's `prefer-destructuring` configuration. Both default to `{array: true, object: true}`. The diagnostic for object destructuring in assignment expressions now instructs users to wrap the assignment in parentheses.
