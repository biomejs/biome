---
"@biomejs/biome": patch
---

Added support for the Svelte syntax `{#if}{/if}`. The Biome HTML parser is now able to parse and format the [`{#if}{/if} blocks`](https://svelte.dev/docs/svelte/if):

```diff
<!-- if / else-if / else -->
{#if porridge.temperature > 100}
-<p>too hot!</p>
+  <p>too hot!</p>
{:else if 80 > porridge.temperature}
-<p>too cold!</p>
+  <p>too cold!</p>
{:else if 100 > porridge.temperature}
-<p>too too cold!</p>
+  <p>too too cold!</p>
{:else}
-<p>just right!</p>
+  <p>just right!</p>
{/if}
```
