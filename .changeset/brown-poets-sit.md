---
"@biomejs/biome": patch
---

Added support Svelte syntax `{@render}`. Biome now is able to parse and format the Svelte syntax [`{@render}`](https://svelte.dev/docs/svelte/@render):

```diff
-{@render   sum(1, 2)   }
+{@render sum(1, 2)}
```

The contents of the expressions inside the `{@render <expression>}` aren't formatted yet.
