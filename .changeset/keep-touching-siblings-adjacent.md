---
"@biomejs/biome": patch
---

Fixed unwanted line breaks between touching HTML siblings, including Svelte blocks, render tags and self-closing elements. These breaks could introduce rendered spaces with `whitespaceSensitivity: "strict"`.

```diff
  {#snippet parts()}
  	{#if leftIcon}
  		<span class="icon">{@render leftIcon()}</span>
- 	{/if}{@render children?.()}
- 	{#if rightIcon}
+ 	{/if}{@render children?.()}{#if rightIcon}
  		<span class="icon">{@render rightIcon()}</span>
  	{/if}
  {/snippet}
```
