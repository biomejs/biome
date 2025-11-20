---
"@biomejs/biome": patch
---

Added support Svelte syntax `{@const}`. Biome now is able to parse and format the Svelte syntax [`{@const}`](https://svelte.dev/docs/svelte/@const):

```diff
-{@const   name = value}
+{@const name = value}
```

The contents of the expressions inside the `{@const <expression>}` aren't formatted yet.
