---
"@biomejs/biome": patch
---

Fixed [#12502](https://github.com/biomejs/biome/issues/12502): [`noInvalidGridAreas`](https://biomejs.dev/linter/rules/no-invalid-grid-areas/) now counts the cells in each row of `grid-template-areas` instead of comparing the length of the row text. Rows such as `"a" "bb"` are no longer reported, and rows such as `"abc" "d e"` are now reported.

