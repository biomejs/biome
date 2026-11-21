---
"@biomejs/biome": patch
---

Fixed [#13335](https://github.com/biomejs/biome/issues/13335): Svelte `{#snippet}` blocks with type parameters, such as `{#snippet row<T extends string>(item: T)}`, are now parsed and formatted. Invalid snippet signatures no longer make the linter panic.
