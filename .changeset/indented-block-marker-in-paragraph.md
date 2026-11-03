---
"@biomejs/biome": patch
---

Fixed [#12757](https://github.com/biomejs/biome/issues/12757): the Markdown formatter no longer turns an indented paragraph line that starts with `-`, `1.`, `#`, or `>` into a list, heading, or blockquote. Such a line now joins the previous line, matching Prettier.

```diff
-- Item
-      - Nested item
+- Item - Nested item
   continuation line
```
