---
"@biomejs/biome": patch
---

Added support for Astro's fragment shorthand, and fixed [#9165](https://github.com/biomejs/biome/issues/9165): an empty expression no longer fails to parse.

```astro
<>
  <p>{}</p>
</>
```
