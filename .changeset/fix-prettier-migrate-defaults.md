---
"@biomejs/biome": patch
---

Fixed [#12265](https://github.com/biomejs/biome/issues/12265): `biome migrate prettier` no longer uses the wrong defaults when the Prettier configuration doesn't set `semi` or `singleQuote`. The migrated configuration now uses `"semicolons": "always"` and `"quoteStyle": "double"`, matching Prettier's defaults.
