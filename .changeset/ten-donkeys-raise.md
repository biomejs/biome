---
"@biomejs/biome": patch
---

Fixed [#7604](https://github.com/biomejs/biome/issues/7604): the `useMaxParams` rule now highlights parameter lists instead of entire function bodies. This provides more precise error highlighting. Previously, the entire function was highlighted; now only the parameter list is highlighted, such as `(a, b, c, d, e, f, g, h)`.
