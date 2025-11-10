---
"@biomejs/biome": patch
---

Added support Svelte syntax `{@attach}`. Biome now is able to parse and format the Svelte syntax [`{@attach}`](https://svelte.dev/docs/svelte/@attach):

```diff
-<div {@attach    myAttachment   }>...</div>
+<div {@attach myAttachment}>...</div>
```
The contents of the expressions inside the `{@attach <expression>}` aren't formatted yet.
