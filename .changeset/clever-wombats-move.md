---
"@biomejs/biome": patch
---

Fixed an issue where, occassionally, some bindings and reference weren't properly tracked, causing some false positives in some lint rules such as [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) and [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) in Svelte, Vue and Astro files.
