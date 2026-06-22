---
"@biomejs/biome": patch
---

Fixed CSS formatter output for comments between import media queries.

```diff
-@import url("print.css") print,
-/* comment */
-screen;
+@import url("print.css") print, /* comment */ screen;
```
