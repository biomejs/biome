---
"@biomejs/biome": patch
---

Fixed the SCSS formatter joining comma-separated multi-value declarations onto one line. SCSS now prints them one value per line, as the CSS formatter and Prettier do.

```diff
 .a {
-	transition: opacity 1s, color 2s;
+	transition:
+		opacity 1s,
+		color 2s;
 }
```
