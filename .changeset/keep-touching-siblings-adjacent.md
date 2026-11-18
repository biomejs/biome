---
"@biomejs/biome": patch
---

Fixed [#13299](https://github.com/biomejs/biome/issues/13299): the HTML formatter no longer adds line breaks between touching siblings such as Svelte blocks, render tags and self-closing elements. With `whitespaceSensitivity: "strict"` these breaks rendered as spaces.

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
