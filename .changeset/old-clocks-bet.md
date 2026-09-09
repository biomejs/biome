---
"@biomejs/biome": patch
---

Fixed parsing of unquoted CSS URLs beginning with `@` or `!`, such as `url(@/assets/icon.svg)` and `url(!font.woff2)`. Preserved escaped and non-ASCII whitespace in raw URLs during formatting.

```diff
-background-image: url(image\);
+background-image: url(image\ );
```
