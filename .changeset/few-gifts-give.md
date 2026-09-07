---
"@biomejs/biome": patch
---

Fixed [#7479](https://github.com/biomejs/biome/issues/7479). [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) now treats Unicode escapes in identifiers as the same binding as their decoded spelling.
