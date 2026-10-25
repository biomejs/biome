---
"@biomejs/biome": patch
---

Fixed [#12643](https://github.com/biomejs/biome/issues/12643) and [#12669](https://github.com/biomejs/biome/issues/12669): Biome now migrates the Prettier option `bracketSpacing` to `formatter.bracketSpacing`, both at the top level and inside `overrides`. JSON and GraphQL files are now formatted with the same bracket spacing as JavaScript files.
