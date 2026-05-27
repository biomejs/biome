---
"@biomejs/biome": patch
---

Fixed [#10330](https://github.com/biomejs/biome/issues/10330): Vue interpolations that contain newline-delimited content now keep a multiline layout instead of being collapsed into whitespace-sensitive tag borrowing.
