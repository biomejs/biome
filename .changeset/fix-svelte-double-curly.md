---
"@biomejs/biome": patch
---

Fixed Svelte files failing to parse when an expression begins with an object literal.

Now the following snippet is correctly parsed:

```svelte
<p>{{ a: true }}</p>
<div class={{ active: isActive }}></div>
```
