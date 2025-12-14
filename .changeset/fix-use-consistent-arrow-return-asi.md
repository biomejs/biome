---
"@biomejs/biome": patch
---

Fixed [#8179](https://github.com/biomejs/biome/issues/8179): [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/) autofix no longer produces semantically incorrect code when adding braces to arrow functions with multiline expressions. Previously, the fix could place a newline between `return` and the expression, triggering JavaScript's automatic semicolon insertion (ASI) and causing the function to return `undefined` instead of the intended value.
