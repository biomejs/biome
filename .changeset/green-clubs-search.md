---
"@biomejs/biome": patch
---

Added support for parsing and formatting the Svelte `{#each}` syntax, when `html.experimentalFullSupportEnabled` is set to `true`.

```diff
- {#each items   as item  }
+ {#each items as item}

{/each}
```
