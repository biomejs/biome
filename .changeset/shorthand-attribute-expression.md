---
"@biomejs/biome": patch
---

Fixes a parser error for Astro shorthand attributes that use an expression instead of a bare identifier, such as `{object?.something}`:

```astro
<div {object?.something}></div>
```
