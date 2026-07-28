---
"@biomejs/biome": patch
---

Fixed CSS formatting of long block comments between comma-separated property values:

```diff
 .foo {
   box-shadow:
-    1000px /* long long long long long long long long long long long long comment */ 1000px /* long long long long long long long long long comment */ 2px color(srgb 0.555555555 0.555555555 0.555555555),
+    1000px
+      /* long long long long long long long long long long long long comment */
+      1000px /* long long long long long long long long long comment */ 2px
+      color(srgb 0.555555555 0.555555555 0.555555555),
     1px 1px black;
 }
```
