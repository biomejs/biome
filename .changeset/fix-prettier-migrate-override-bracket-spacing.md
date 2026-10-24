---
"@biomejs/biome": patch
---

Fixed [#12643](https://github.com/biomejs/biome/issues/12643): Biome now migrates the Prettier option `bracketSpacing` when it is set inside `overrides`. The option is migrated to the `formatter.bracketSpacing` of the matching Biome override.
