---
"@biomejs/biome": patch
---

Fixed Svelte `{#each}` parser incorrectly rejecting TypeScript `as const` type assertions in the iterable expression. Biome now correctly parses `{#each arr as const as item}`.
