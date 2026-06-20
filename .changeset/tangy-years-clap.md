---
"@biomejs/biome": minor
---

Added support for Svelte's markup `{let ...}` and `{const ...}` declaration blocks, including multiple declarations and destructuring. Biome now parses and formats them instead of emitting a parse error.

```svelte
{#each boxes as box}
	{let area = box.width * box.height}
{/each}

{const a = 1, b = 2, c = 3}
```
