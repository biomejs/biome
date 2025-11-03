---
"@biomejs/biome": patch
---

Added support for the Svelte syntax `{@debug}`. The Biome HTML parser is now able to parse and format the blocks:

```diff
-{@debug     foo,bar,    something}
+{@debug foo, bar, something}
```
