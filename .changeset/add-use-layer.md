---
"@biomejs/biome": patch
---

Added the nursery rule [`useLayeredStyles`](https://biomejs.dev/linter/rules/use-layered-styles/), which enforces that style rules are defined within a cascade layer and import rules to import its styles into a cascade layer.

```css
/* Invalid */
@import 'foo.css';

.my-style {
  color: red;
}

/* Valid */
@import 'foo.css' layer(base);

@layer base {
  .my-style {
    color: red;
  }
}
```
