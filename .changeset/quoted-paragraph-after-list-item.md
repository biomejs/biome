---
"@biomejs/biome": patch
---

Fixed [#12703](https://github.com/biomejs/biome/issues/12703): the Markdown formatter no longer indents the empty `>` line between a list and a paragraph in a blockquote. Formatting again no longer moves the paragraph out of the quote.

```diff
 > - item one
 >   continued.
-  >
+>
 > Paragraph line one
 > line two.
```
