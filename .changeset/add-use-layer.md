---
"@biomejs/biome": patch
---

Added the nursery rule [`useLayeredStyles`](https://biomejs.dev/linter/rules/use-layered-styles/), which enforces that CSS style rules are defined within a cascade layer.

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
