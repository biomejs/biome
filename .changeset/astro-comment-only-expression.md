---
"@biomejs/biome": patch
---

Fixed an Astro expression holding only a comment being reported as a parse error, which also stopped the whole file from being formatted.

```astro
<div>{/* a note */}</div>
<div class={/* a note */}>x</div>
```
