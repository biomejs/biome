---
"@biomejs/biome": patch
---

Fixed [#8409](https://github.com/biomejs/biome/issues/8409): CSS formatter now correctly places comments after the colon in property declarations.

Previously, comments that appeared after the colon in CSS property values were incorrectly moved before the property name:

```diff
[lang]:lang(ja) {
-  /* system-ui,*/ font-family:
+  font-family: /* system-ui,*/
    Hiragino Sans,
    sans-serif;
}
```
