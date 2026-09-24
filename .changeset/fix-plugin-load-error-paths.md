---
"@biomejs/biome": patch
---

Fixed [#10846](https://github.com/biomejs/biome/issues/10846): when plugins fail to load, Biome now prints each failing plugin's path on its own line, instead of concatenating bare messages like `Cannot read file.Cannot read file.`.
