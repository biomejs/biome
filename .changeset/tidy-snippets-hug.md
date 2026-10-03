---
"@biomejs/biome": patch
---

Fixed [#12079](https://github.com/biomejs/biome/issues/12079): the formatter no longer puts the parameters of a Svelte `{#snippet}` block on separate lines when they fit on one line.

```diff
-{#snippet children(
-	item,
-)}
+{#snippet children(item)}
```
