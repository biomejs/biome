---
"@biomejs/biome": patch
---

Fixed [#12541](https://github.com/biomejs/biome/issues/12541): Markdown list items whose markers are indented 4 or more spaces now nest at the correct level. Previously the formatter could move such an item under the item before it, or join it to that item's text.

```diff
 1. Step
    - a
      - x
-       - y
-         - b
+     - y
+   - b
```
