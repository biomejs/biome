---
"@biomejs/biome": patch
---

Added the nursery rule [`useSvelteShorthandDirective`](https://biomejs.dev/linter/rules/use-svelte-shorthand-directive/), which enforces the shorthand syntax for Svelte `bind:`, `class:`, and `style:` directives. For example, `<input bind:value={value} />` is now reported and can be fixed to `<input bind:value />`.
