---
"@biomejs/biome": patch
---

Fixed [#9253](https://github.com/biomejs/biome/issues/9253): parsing of `@container scroll-state(...)` queries.

```css
@container scroll-state(scrolled: bottom) { }
@container scroll-state(stuck) { }
@container scroll-state(not (stuck)) { }
@container scroll-state((stuck) and (scrolled: bottom)) { }
@container scroll-state((stuck) or (snapped: x)) { }
@container main-layout scroll-state(not ((stuck) and (scrolled: bottom))) { }
```
