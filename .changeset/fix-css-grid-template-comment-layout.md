---
"@biomejs/biome": patch
---

Fixed CSS formatting for `grid-template-areas` declarations with comments before multiline values. Biome now keeps grid area rows aligned instead of adding an extra declaration-boundary indent.

```diff
 .grid {
   grid-template-areas:
 /* row */
-      "header header"
-      "footer footer";
+    "header header"
+    "footer footer";
 }
```
