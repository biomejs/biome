---
"@biomejs/biome": patch
---

Added the nursery rule [`noZeroLengthUnit`](https://biomejs.dev/linter/rules/no-zero-length-unit/), which disallows units on zero lengths in CSS. Biome can remove the unit.

```css
a {
  top: 0px;
}
```
