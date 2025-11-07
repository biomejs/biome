---
"@biomejs/biome": patch
---


Added support Svelte syntax `{@html}`. Biome now is able to parse and format the Svelte syntax [`{@html}`](https://svelte.dev/docs/svelte/@html):

```diff
-{@html   'div'}
+{@html 'div'}
```
The contents of the expressions inside the `{@html <expression>}` aren't formatted yet.
