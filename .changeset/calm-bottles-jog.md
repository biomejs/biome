---
"@biomejs/biome": patch
---

Added a new nursery rule [`useNamedLayer`](https://biomejs.dev/linter/rules/use-named-layer) which disallows anonymous cascade layers.

```css
@layer {
  a {
    color: red;
  }
}
```
