---
"@biomejs/biome": patch
---

Added the nursery rule [`noMissingScssInterpolation`](https://biomejs.dev/linter/rules/no-missing-scss-interpolation/), which disallows Sass variables and function calls in custom property values without `#{}` interpolation. Sass copies these values into the CSS unchanged, so the browser receives `$gap` or `math.div(8px, 2)` instead of their results. Its unsafe fix wraps variables in `#{}`.

```scss
:root {
  --gap: $gap;
  --half: math.div(8px, 2);
}
```
