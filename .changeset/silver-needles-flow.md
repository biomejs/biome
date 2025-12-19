---
"@biomejs/biome": patch
---

Added support for parsing and formatting the Svelte `{#snippet}` syntax, when `html.experimentalFullSupportEnabled` is set to `true`.

```diff
-{#snippet    foo() }
+{#snippet foo()}

{/snippe}
```
