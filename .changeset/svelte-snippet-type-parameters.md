---
"@biomejs/biome": patch
---

Fixed [#13335](https://github.com/biomejs/biome/issues/13335): Svelte snippets with type parameters, such as `{#snippet row<T extends string>(item: T)}`, are now parsed correctly. Previously, Biome reported a parse error, and the linter panicked on these files.
