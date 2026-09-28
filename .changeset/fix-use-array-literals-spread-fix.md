---
"@biomejs/biome": patch
---

Fixed [#11942](https://github.com/biomejs/biome/issues/11942): [`useArrayLiterals`](https://biomejs.dev/linter/rules/use-array-literals/) no longer offers a safe fix for calls such as `Array(...args)` or `new Array(x, ...rest)`. A spread may leave a single numeric argument, which creates an array of that length rather than an array containing it, so these calls are still reported but are no longer rewritten to an array literal.
