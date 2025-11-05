---
"@biomejs/biome": patch
---

Fixed [#7981](https://github.com/biomejs/biome/issues/7981). Now Biome correctly detects and parses `lang='tsx'` and `lang='jsx'` languages when used inside in `.vue` files, when  `.experimentalFullSupportEnabled` is enabled.
