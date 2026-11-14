---
"@biomejs/biome": patch
---

Fixed [#13167](https://github.com/biomejs/biome/issues/13167): The HTML formatter no longer deletes comments that follow a Svelte block delimiter on the same line, such as `{#if}`, `{:else}`, or `{#each}`. For example, `{#if a}<!-- comment --><i>x</i>{/if}` is now formatted as:

```diff
  {#if a}
- 	<i>x</i>
+ 	<!-- comment --><i>x</i>
  {/if}
```
