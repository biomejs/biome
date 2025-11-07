---
"@biomejs/biome": patch
---

The CSS parser, with `tailwindDirectives` enabled, will now accept lists of selectors in `@custom-variant` shorthand syntax.

```css
@custom-variant cell (th:has(&), td:has(&));
```
