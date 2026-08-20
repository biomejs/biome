---
"@biomejs/biome": patch
---

Fixed Astro rejecting JavaScript comments between attributes.

```astro
<div /* block comment */ class="something"></div>
<Component /* c */ client:load />
```
