---
"@biomejs/biome": patch
---

Fixed an Astro expression holding only a comment failing to parse.

```astro
<div>{/* a note */}</div>
```
