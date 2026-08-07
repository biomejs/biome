---
"@biomejs/biome": patch
---

Fixed [#11228](https://github.com/biomejs/biome/issues/11228): CSS block comments between a declaration colon and value now preserve their source indentation.

```diff
 :root {
   --font-stack:
-/* comment */
+    /* comment */
     system-ui;
 }
```
