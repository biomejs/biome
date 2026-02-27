---
"@biomejs/biome": patch
---

Fixed [#8409](https://github.com/biomejs/biome/issues/8409): CSS formatter now correctly places comments after the colon in property declarations.

Previously, comments that appeared after the colon in CSS property values were incorrectly moved before the property name:

```css
/* Before (incorrect) */
[lang]:lang(ja) {
  /* system-ui,*/ font-family:
    Hiragino Sans,
    sans-serif;
}

/* After (correct) */
[lang]:lang(ja) {
  font-family: /* system-ui,*/
    Hiragino Sans,
     sans-serif;
}
```
