---
"@biomejs/biome": patch
---

Added the new nursery rule [`useScssColorChannel`](https://biomejs.dev/linter/rules/use-scss-color-channel/). It reports the older Sass functions that read one channel of a color, such as `red()`, `color.hue()` and `opacity()`, and suggests `color.channel()` instead.

```scss
a {
  opacity: opacity($brand);
}
```
