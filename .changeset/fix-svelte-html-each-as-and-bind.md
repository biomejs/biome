---
"@biomejs/biome": patch
---

Fixed [#10839](https://github.com/biomejs/biome/issues/10839): Svelte formatter now matches Prettier for `{#each list as [item]}` (no spaces inside array destructuring brackets) and for `bind:prop={get, set}` line breaking.
