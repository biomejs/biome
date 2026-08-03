---
"@biomejs/biome": patch
---

Fixed [#11171](https://github.com/biomejs/biome/issues/11171): variables referenced only inside a Svelte attachment (`{@attach ...}`) are no longer reported as unused by [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) and [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/).
