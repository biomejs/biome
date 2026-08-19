---
"@biomejs/biome": patch
---

Fixed [#9165](https://github.com/biomejs/biome/issues/9165): an empty Astro expression no longer fails to parse. Astro renders `{}` as nothing.

```astro
<div>{}</div>
```
