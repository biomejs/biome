---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicateElseIf`](https://biomejs.dev/linter/rules/no-duplicate-else-if/) for Svelte templates: disallow duplicate conditions in `{#if}` / `{:else if}` chains. A condition identical to a previous one can never execute:

```svelte
{#if a}
  <div>a</div>
{:else if a}
  <div>unreachable</div>
{/if}
```
