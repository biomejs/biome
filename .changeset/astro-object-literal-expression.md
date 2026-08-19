---
"@biomejs/biome": patch
---

Fixed `{{` at the start of an Astro expression being read as an interpolation. Astro has no `{{ }}` syntax, so `{{ a: 1 }}` and `<Comp a={{ b: 1 }} />` are object literals.
