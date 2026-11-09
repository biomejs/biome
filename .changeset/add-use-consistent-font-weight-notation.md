---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentFontWeightNotation`](https://biomejs.dev/linter/rules/use-consistent-font-weight-notation/), which enforces writing font weights either as numbers or as the keywords `normal` and `bold` where possible. It checks the `font-weight` and `font` properties, and its safe fix converts between `normal`/`bold` and `400`/`700`.

```css
a { font-weight: bold; }
```
