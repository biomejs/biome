---
"@biomejs/biome": patch
---

Fixed [#11927](https://github.com/biomejs/biome/issues/11927): The HTML formatter no longer duplicates comments around Svelte blocks. This affected a comment after a block such as `{#if}` or `{#each}` at the end of an element, and a comment on the same line as the last element inside a block, before `{:else}`, `{/if}`, or a similar tag.
