---
"@biomejs/biome": patch
---

Fixed [#12953](https://github.com/biomejs/biome/issues/12953): the Markdown formatter no longer removes the blank line between a blockquote that ends a list item and the block that follows it, matching Prettier. Removing that line could also move a following paragraph into the blockquote. The same applies to a list item that ends with a hard line break.

```diff
 - a
   > b
+
 #### Heading
```
