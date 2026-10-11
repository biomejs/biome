---
"@biomejs/biome": patch
---

Fixed [#12669](https://github.com/biomejs/biome/issues/12669): Biome now migrates the Prettier option `bracketSpacing` to the top-level `formatter.bracketSpacing`, so JSON and GraphQL files are formatted with the same bracket spacing as JavaScript files.
