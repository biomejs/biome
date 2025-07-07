---
"@biomejs/biome": patch
---

Improved handling of multiple adjacent line suppressions (fixed [#6621](https://github.com/biomejs/biome/issues/6621)). Biome now handles such suppressions separately, tracking whether each one is used.
