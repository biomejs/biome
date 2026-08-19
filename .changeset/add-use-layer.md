---
"@biomejs/biome": patch
---

Added the nursery rule [`useLayer`](https://biomejs.dev/linter/rules/use-layer/), which enforces that CSS style rules are defined within a cascade layer.

```css
/* Invalid */
.my-style {
  color: red;
}

/* Valid */
@layer base {
  .my-style {
    color: red;
  }
}
```
