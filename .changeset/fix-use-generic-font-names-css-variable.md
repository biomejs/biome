---
"@biomejs/biome": patch
---

Fix [`useGenericFontNames`](https://biomejs.dev/linter/rules/use-generic-font-names/) false positive when a CSS variable is used as the last value in `font-family` or `font`. The rule now correctly ignores cases like `font-family: "Noto Serif", var(--serif)` and `font: 1em Arial, var(--fallback)`.
