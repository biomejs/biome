---
"@biomejs/biome": patch
---

Fixed CSS formatting for multiline function arguments preceded by comments:

```diff
 .example {
   value: outer(
     1,
     /* comment */
     nested(
-      first,
-      second
-    )
+        first,
+        second
+      )
   );
 }
```
