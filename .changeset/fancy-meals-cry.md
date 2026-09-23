---
"@biomejs/biome": patch
---

Fixed [#11504](https://github.com/biomejs/biome/issues/11504), a regression where Biome would silently ignore errors in the configuration file. Now errors are correctly retained and checked before executing any command.
