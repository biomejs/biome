---
"@biomejs/biome": patch
---

Fixed a bug in the LSP file watcher registration so Biome now watches `.biome.json` and `.biome.jsonc` configuration files and reloads workspace settings when they change.
