---
"@biomejs/biome": patch
---

Fixed [#12265](https://github.com/biomejs/biome/issues/12265): Biome now uses the correct defaults for `semi` and `singleQuote` when migrating a Prettier configuration that doesn't set them. The migrated configuration uses `"semicolons": "always"` and `"quoteStyle": "double"`, matching Prettier's defaults.
