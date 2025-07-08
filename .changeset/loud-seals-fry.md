---
"@biomejs/biome": patch
---

Fixed [#6537](https://github.com/biomejs/biome/issues/6537): Biome no longer
removes the trailing comma from JSON files when
`formatter.json.trailingCommas` is explicitly set to `"all"`.
