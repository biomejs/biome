---
"@biomejs/biome": patch
---

Fixed the Markdown parser keeping a paragraph inside a nested blockquote after a line that contains only the `>` of an outer blockquote. Such a line now ends the nested blockquote, so the formatter no longer moves the following paragraph into it.

```diff
 > > quote
 >
-> > paragraph
+> paragraph
```
