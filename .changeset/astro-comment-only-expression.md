---
"@biomejs/biome": patch
---

Fixed [#8294](https://github.com/biomejs/biome/issues/8294): an Astro expression holding only a comment is no longer reported as a parse error, which also stopped the whole file from being formatted.

```astro
<div>{/* a note */}</div>
<div class={/* a note */}>x</div>
```
