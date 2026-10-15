---
"@biomejs/biome": patch
---

Fixed parsing and formatting of `at-rule()` supports queries in CSS and SCSS. Balanced fallback queries retain their token gaps, and suppressed CSS retains literal Unicode line and paragraph separators. Hex escapes followed by CRLF now parse correctly.

```css
@supports at-rule(@function) {
  .card { padding: --double(8px); }
}
```
