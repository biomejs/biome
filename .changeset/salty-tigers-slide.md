---
"@biomejs/biome": patch
---

Fixed [#6621](https://github.com/biomejs/biome/issues/6621): Improved handling
of multiple adjacent line suppressions. Biome now handles such suppressions
separately, tracking whether each one is used.
