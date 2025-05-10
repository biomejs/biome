---
"@biomejs/biome": patch
---

Fixed [#5919](https://github.com/biomejs/biome/issues/5919). Now Biome correctly loads the configuration passed via `--config-path` when its path starts with `./` e.g. `--confi-path=./project/biome.json`
