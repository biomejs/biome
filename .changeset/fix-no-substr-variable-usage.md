---
"@biomejs/biome": patch
---

Fixed [#9279](https://github.com/biomejs/biome/issues/9279): The rule [`noSubstr`](https://biomejs.dev/linter/rules/no-substr/) now detects `.substr()` and `.substring()` calls in all expression contexts, including variable declarations, function arguments, return statements, and arrow function bodies.
