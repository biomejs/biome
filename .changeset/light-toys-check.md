---
"@biomejs/biome": patch
---

Added support Svelte syntax `{#key}`. Biome now is able to parse and format the Svelte syntax [`{#key}`](https://svelte.dev/docs/svelte/key):

```diff
-{#key   expression} <div></div> {/key}
+{#key expression}
+  <div></div>
+{/key}
```
The contents of the expressions inside the `{#key <expression>}` aren't formatted yet.
