---
"@biomejs/biome": patch
---

Fixed formatting of Markdown block quotes inside deeply indented list items. With `proseWrap: "always"`, consecutive quote lines now wrap together within the same quote, while setext headings and fenced code blocks keep their structure.

```diff
 10. item
-    > first line
-    > second line
+    > first line second line
```
