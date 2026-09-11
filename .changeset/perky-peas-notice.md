---
"@biomejs/biome": patch
---

Fixed redundant parentheses around binary and logical unary operands with leading line comments.

```diff
 !(
   // leading
-  (a || b)
+  a || b
 );
```
