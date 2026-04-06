---
"@biomejs/biome": patch
---

Added a new nursery CSS rule [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/), that disallows duplicate selector lists within the same at-rule context.

For example, the following snippet triggers the rule because the second selector and the first selector are the same:

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
