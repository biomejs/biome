---
"@biomejs/biome": patch
---

Fixed [#12767](https://github.com/biomejs/biome/issues/12767): the Markdown formatter no longer moves a list item out of a blockquote when the item follows a nested list item. The formatter also keeps the `>` of a line that directly follows a list inside a blockquote and starts a nested blockquote, a heading, or a different list.

```diff
 > - one
 >   - nested
-  - two
+> - two
```
