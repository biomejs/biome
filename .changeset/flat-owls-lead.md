---
"@biomejs/biome": patch
---

Fixed [#11950](https://github.com/biomejs/biome/issues/11950): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) now reports arrow functions with expression bodies that only reference themselves, such as `let h = () => h();`.
