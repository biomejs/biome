---
"@biomejs/biome": patch
---

Fixed [#12389](https://github.com/biomejs/biome/issues/12389): the Svelte formatter no longer duplicates binding names when converting redundant `bind:` initializers to shorthand syntax.

```diff
-<input bind:valuevalue>
+<input bind:value>
```
