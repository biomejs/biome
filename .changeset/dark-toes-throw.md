---
"@biomejs/biome": patch
---

Fixed [#8347](https://github.com/biomejs/biome/issues/8347): the fix from [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/) now parenthesizes returned expressions that begin with object literals before removing the arrow function body braces, preventing invalid output for expressions such as object property access.
