---
"@biomejs/biome": patch
---

Fixed an issue where, occasionally, some bindings and references were not properly tracked, causing false positives from [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) and [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) in Svelte, Vue, and Astro files.
