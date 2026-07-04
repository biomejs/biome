---
"@biomejs/biome": patch
---

Fixed [#10550](https://github.com/biomejs/biome/issues/10550): added support for Svelte's markup `{let ...}` and `{const ...}` declaration blocks ([RFC](https://github.com/sveltejs/svelte/issues/16490)), including multiple declarations and destructuring. Biome now parses and formats them instead of emitting a parse error, and resolves the bindings they introduce.

```svelte
{#each boxes as box}
	{let area = box.width * box.height}
{/each}

{const a = 1, b = 2, c = 3}
```
