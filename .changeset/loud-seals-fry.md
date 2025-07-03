---
"@biomejs/biome": patch
---

Fixed [#6537](https://github.com/biomejs/biome/issues/6537), where Biome removed the trailing comma from JSON files even when `formatter.json.trailingCommas` is explicitly set to `"all"`.
