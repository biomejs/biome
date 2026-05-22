---
"@biomejs/biome": patch
---

Fixed CSS and SCSS formatting for comments around declaration colons so comments between property names, colons, and values stay at the same boundary as Prettier.

```diff
 .selector {
-  color: /* red, */
-    blue;
+  color: /* red, */ blue;
 }
```
