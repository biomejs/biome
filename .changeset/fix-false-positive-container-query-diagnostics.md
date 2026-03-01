---
"@biomejs/biome": patch
---

Fixed [#9253](https://github.com/biomejs/biome/issues/9253): removed false-positive diagnostics for valid `@container`/`@supports` general-enclosed queries.

```css
@container scroll-state(scrolled: bottom) { }
@supports foo(bar: baz) { }
```
