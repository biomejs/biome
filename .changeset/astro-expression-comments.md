---
"@biomejs/biome": patch
---

Fixed Astro expressions containing a comment failing to parse.

```astro
<div>{/* block comment */ x}</div>
<div>{/* only a comment */}</div>
```
