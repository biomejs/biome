---
"@biomejs/biome": patch
---

Fixed the Svelte `{#each}` parser incorrectly stopping at TypeScript `as const` type assertions in the iterable expression (e.g. `{#each arr as const as item}`). Fixed the CSS parser rejecting comma-separated selector lists inside `:global()` and `:local()` pseudo-class functions (e.g. `:global(.foo, .bar)`).
