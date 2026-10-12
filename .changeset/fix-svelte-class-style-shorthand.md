---
"@biomejs/biome": patch
---

Fixed [#11215](https://github.com/biomejs/biome/issues/11215): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports variables in Svelte components that are only used through the `class:` and `style:` directive shorthands, such as `<div class:active>` and `<div style:color>`.
