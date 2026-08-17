---
"@biomejs/biome": patch
---

Fixed HTML text wrapping to account for the width of an adjacent closing tag, avoiding lines that exceed the configured width when the final word and tag must move together.

```diff
 <a-long-long-long-element
-  >foo bar foo bar foo bar foo bar foo bar foo bar foo bar</a-long-long-long-element
+  >foo bar foo bar foo bar foo bar foo bar foo
+  bar</a-long-long-long-element
 >
```
