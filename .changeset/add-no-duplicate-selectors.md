---
"@biomejs/biome": patch
---

Added a new nursery CSS rule `noDuplicateSelectors`, that disallows duplicate selector lists within the same at-rule context.

For example, the following snippet trigges the rule because the second selector and the first selector are the same:

```css
/* First selector */
.x .y .z {}

/* Second selector */
.x {
  .y {
    .z {}
  }
}
```
