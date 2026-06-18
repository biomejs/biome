---
"@biomejs/biome": patch
---

Fixed a false positive in [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) for Svelte files where variables referenced inside `{@html expr}` blocks were incorrectly reported as unused.
