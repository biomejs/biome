---
"@biomejs/biome": patch
---

Fixed [#9098](https://github.com/biomejs/biome/issues/9098): `useImportType` no longer incorrectly flags imports used in Svelte control flow blocks (`{#if}`, `{#each}`, `{#await}`, `{#key}`) as type-only imports.
