---
"@biomejs/biome": patch
---

Fixed [#8845](https://github.com/biomejs/biome/issues/8845): `useGenericFontNames` rule now correctly skips `font-family` declarations inside `@supports` at-rules.

The rule was incorrectly flagging font-family feature detection queries as missing generic font names:

```css
/* No longer reports an error */
@supports (font-family: "Test Font") {
    .test {
        font-family: "Test Font", sans-serif;
    }
}
```
