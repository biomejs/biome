---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentLightnessNotation`](https://biomejs.dev/linter/rules/use-consistent-lightness-notation/), which enforces writing the lightness of `lab()`, `lch()`, `oklab()`, and `oklch()` colors as a percentage, or as a number with the `notation` option. For example, with the default options, `oklch(0.85 0.17 88)` is reported and can be fixed to `oklch(85% 0.17 88)`.
