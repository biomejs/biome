---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteUselessDerivedBy`](https://biomejs.dev/linter/rules/no-svelte-useless-derived-by/), which reports `$derived.by()` calls whose function only returns a single expression, such as `$derived.by(() => count * 2)`, and suggests `$derived(count * 2)` instead.
