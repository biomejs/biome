---
"@biomejs/biome": minor
---

Added the formatter option `markdown.formatter.unorderedListMarker`, which sets the marker of unordered list items to `dash` (`-`, the default), `asterisk` (`*`), or `plus` (`+`). When two unordered lists are next to each other, the second list still uses another marker so that Markdown doesn't merge them into one list.

```diff
- - first item
- - second item
+ * first item
+ * second item
```
