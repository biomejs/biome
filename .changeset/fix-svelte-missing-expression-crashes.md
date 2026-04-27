---
"@biomejs/biome": patch
---

Fixed [#10135](https://github.com/biomejs/biome/issues/10135): Biome no longer crashes on missing Svelte template expressions.

The following code snippet longer panics:
```svelte
{#if }
 <p>^ this would previously crash</p>
{/if}
{@const }
<p>    ^ this would also crash</p>
```
