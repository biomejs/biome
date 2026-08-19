---
"@biomejs/biome": patch
---

Fixed `{{` at the start of an Astro expression being read as an interpolation. Astro has no `{{ }}` syntax, so this is an object literal.

```astro
{{ a: 1 }}
<Comp a={{ b: 1 }} />
```
