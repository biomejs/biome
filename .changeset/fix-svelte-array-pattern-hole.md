---
"@biomejs/biome": patch
---

Fixed the HTML formatter refusing to format a Svelte file containing an array pattern that skips a position:

```svelte
{#each animals as [, value]}
	<p>{value}</p>
{/each}
```

