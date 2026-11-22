---
"@biomejs/biome": patch
---

Fixed [#13334](https://github.com/biomejs/biome/issues/13334): the Markdown formatter now uses `indentWidth` to indent the content of list items, as Prettier does with `tabWidth`. Some content keeps its current alignment, such as blockquotes, HTML blocks, and lists inside blockquotes.

With `indentWidth` set to `4`:

```diff
 - parent

-  second paragraph
-  - child
+    second paragraph
+    - child
```
