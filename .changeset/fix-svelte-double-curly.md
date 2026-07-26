---
"@biomejs/biome": patch
---

Fixed Svelte files failing to parse when an expression begins with an object literal. `{{` was read as the start of an interpolation, as it is in Vue, but Svelte has no such syntax: there the outer brace opens the expression and the inner one opens an object.

```svelte
<p>{{ a: true }}</p>
<div class={{ active: isActive }}></div>
```
