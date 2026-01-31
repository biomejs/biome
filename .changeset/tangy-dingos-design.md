---
"@biomejs/biome": minor
---

Added the ability to load the hidden files `.biome.json` and `.biome.jsonc`. This is the order how Biome will attempt the configuration file is:
1. `biome.json`
2. `biome.jsonc`
3. `.biome.json`
4. `.biome.jsonc`
