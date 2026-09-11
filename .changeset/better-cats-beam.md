---
"@biomejs/biome": patch
---

Fixed [#8471](https://github.com/biomejs/biome/issues/8471): `source.fixAll.biome` ignored `formatter.formatWithErrors`. It now applies safe fixes without formatting files that have parse errors when the option is disabled.
