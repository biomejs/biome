---
"@biomejs/biome": patch
---

Added the nursery rule [`noUninterpolatedScssFunction`](https://biomejs.dev/linter/rules/no-uninterpolated-scss-function/), which disallows Sass function calls in custom property values without `#{}` interpolation. Sass copies these values into the CSS unchanged, so the browser receives the call as plain text instead of its result.

```scss
:root {
  --half: math.div(8px, 2);
}
```
