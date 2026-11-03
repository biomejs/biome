---
"@biomejs/biome": patch
---

Fixed [#12768](https://github.com/biomejs/biome/issues/12768): the Markdown formatter no longer splits a blockquote in two when an empty `>` line separates its list items.

```diff
 > - one
 > - two
-
+>
 > - three
```
