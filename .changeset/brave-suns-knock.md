---
"@biomejs/biome": patch
---

Added support for parsing and formatting the Svelte `{#await}` syntax, when `html.experimentalFullSupportEnabled` is set to `true`.

```diff
-{#await promise  then name }
+{#await promise then name}

-{:catch    name}
+{:catch name}

{/await}
```
