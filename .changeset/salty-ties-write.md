---
"@biomejs/biome": patch
---

Fixed [#12278](https://github.com/biomejs/biome/issues/12278): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) now recognizes Svelte imports used only through `$store` subscriptions in markup or another script block.
