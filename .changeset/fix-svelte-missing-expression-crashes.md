---
"@biomejs/biome": patch
---

No longer crashes on missing Svelte template expression, fixing [#10135](https://github.com/biomejs/biome/issues/10135), [#10003](https://github.com/biomejs/biome/issues/10003).

The following no longer panics:
```sveltehtml
{#if }
 <p>^ this would previously crash</p>
{/if}
{@const }
<p>    ^ this would also crash</p>
```
