---
"@biomejs/biome": patch
---

Fixed Astro rejecting attribute names that start with a colon.

```astro
<a :href="`/${url}`">Home</a>
```
