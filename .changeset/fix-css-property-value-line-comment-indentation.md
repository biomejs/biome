---
"@biomejs/biome": patch
---

Fixed CSS formatting of line comments between a declaration colon and value to preserve their source indentation.

```diff
 .test {
   background:
-  /////// foo
-  // bar
+        /////// foo
+        // bar
     radial-gradient(circle, #000, transparent);
 }
```
