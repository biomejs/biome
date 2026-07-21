---
"@biomejs/biome": patch
---

Fixed CSS formatting for comments between declaration values and `!important`.

```diff
-a { color: /* before */ /* after */ red !important; }
+a { color: /* before */ red /* after */ !important; }
```
