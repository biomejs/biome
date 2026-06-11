---
"@biomejs/biome": patch
---

Fixed the Svelte parser rejecting `{#each}` blocks where the binding uses object destructuring with property renaming, e.g. `{#each items as { id, component: Filter }}`. Biome now correctly parses and formats these rename bindings.
