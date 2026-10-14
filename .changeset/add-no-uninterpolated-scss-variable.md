---
"@biomejs/biome": patch
---

Added the nursery rule [`noUninterpolatedScssVariable`](https://biomejs.dev/linter/rules/no-uninterpolated-scss-variable/), which disallows Sass variables in custom property values without `#{}` interpolation. Sass copies these values into the CSS unchanged, so the browser receives `$gap` instead of the variable's value.

```scss
:root {
  --gap: $gap;
}
```
