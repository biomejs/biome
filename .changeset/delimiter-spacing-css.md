---
"@biomejs/biome": minor
---

Added `delimiterSpacing` support for CSS. When enabled, Biome inserts spaces inside parentheses and square brackets when the content fits on a single line. Empty delimiters are not affected. It can be configured via `css.formatter.delimiterSpacing`. Defaults to false.

```diff
- rgba(0, 0, 0, 1)
+ rgba( 0, 0, 0, 1 )
```

```diff
- [data-attr]
+ [ data-attr ]
```
