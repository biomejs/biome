---
"@biomejs/biome": patch
---

Fixed `v-` prefixed attribute names being read as Vue directives in Astro files, where they are ordinary attribute names.

```astro
<div v-if="z" x-on:keyup.enter="w"></div>
```
