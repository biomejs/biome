---
"@biomejs/biome": patch
---

Fixed [#9189](https://github.com/biomejs/biome/issues/9189): `biome ci` in GitHub Actions now correctly disables colors so that `::error`/`::warning` workflow commands are not wrapped in ANSI escape codes.
