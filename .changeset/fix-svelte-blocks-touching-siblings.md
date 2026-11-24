---
"@biomejs/biome": patch
---

Fixed the HTML formatter adding a line break between a Svelte block (`{#if}`, `{#each}`, `{#await}`, `{#key}`, `{#snippet}`) or tag (`{@const}`, `{let}`, `{@debug}`) and a sibling that touches it in the source. Svelte keeps that line break as a text node, so the page rendered a space that wasn't in the source.

```diff
 <p>
-	Total:
-	{#each items as item}
+	Total:{#each items as item}
 		<b>{item}</b>
-	{/each}
-	.
+	{/each}.
 </p>
```
