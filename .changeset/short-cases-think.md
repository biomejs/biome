---
"@biomejs/biome": patch
---

Fixed [#7256](https://github.com/biomejs/biome/issues/7256): The [`useConsistentCurlyBraces`](https://biomejs.dev/linter/rules/use-consistent-curly-braces/) rule now correctly ignores a string literal with braces that contains only whitespaces. Previously, literals that contains single whitespace were only allowed.
