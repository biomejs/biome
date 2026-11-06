---
"@biomejs/biome": patch
---

Fixed [#12957](https://github.com/biomejs/biome/issues/12957): Biome now correctly parses SCSS negative interpolation list items as separate values instead of an invalid subtraction operator.

```diff
-margin: -#{$s} - #{$s} - #{$s} auto;
+margin: -#{$s} -#{$s} -#{$s} auto;
```
