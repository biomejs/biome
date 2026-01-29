---
"@biomejs/biome": patch
---

Fixed [#8845](https://github.com/biomejs/biome/issues/8845): [`useGenericFontNames`](https://biomejs.dev/linter/rules/use-generic-font-names/) no longer reports a false positive when `font` or `font-family` is used inside `@supports` rules for feature detection.
