---
"@biomejs/biome": patch
---

Fixed [#9358](https://github.com/biomejs/biome/issues/9358) and [#9375](https://github.com/biomejs/biome/issues/9375). Now attributes that have text expressions such as `class={buttonClass()}` are correctly tracked in Svelte files.
