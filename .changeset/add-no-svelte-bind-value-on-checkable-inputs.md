---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteBindValueOnCheckableInputs`](https://biomejs.dev/linter/rules/no-svelte-bind-value-on-checkable-inputs/), which reports `bind:value` on checkbox and radio inputs in Svelte files. Users change whether these inputs are checked, not their `value`, so `bind:value` does not track their choice.

```svelte
<input type="checkbox" bind:value={accepted} />
```
