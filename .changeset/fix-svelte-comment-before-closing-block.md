---
"@biomejs/biome": patch
---

Fixed the indentation of comments at the end of a Svelte block's contents. A comment before `{:else}`, `{:then}`, `{/if}`, or a similar tag is now indented with the block's contents instead of with the tag.

```diff
 {#if condition}
 	<span>Text</span>
-<!-- comment -->
+	<!-- comment -->
 {/if}
```
