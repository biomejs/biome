---
"@biomejs/biome": patch
---

The HTML parser now parses Svelte `{expression}` interpolations inside quoted attribute values into structured nodes, instead of treating the whole value as an opaque string.

```svelte
<div style="top: {top}px" class="card {cls}"></div>
```
