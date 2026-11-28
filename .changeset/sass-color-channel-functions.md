---
"@biomejs/biome": patch
---

Added the new nursery rule [`noScssDeprecatedColorFunctions`](https://biomejs.dev/linter/rules/no-scss-deprecated-color-functions/). It reports the deprecated Sass functions that read one channel of a color, such as `red()`, `color.hue()` and `opacity()`, and suggests `color.channel()` instead.

```scss
a {
  opacity: opacity($brand);
}
```
