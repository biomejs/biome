---
"@biomejs/biome": patch
---

Fixed [#11927](https://github.com/biomejs/biome/issues/11927): The HTML formatter no longer duplicates a comment that follows a Svelte block, such as `{#if}` or `{#each}`, at the end of an element.
