---
"@biomejs/biome": patch
---

Fixed [#113](https://github.com/biomejs/biome-zed/issues/113), where the Biome Language Server didn't correctly update the diagnostics when the configuration file is modified in the editor. Now the diagnostics are correctly updated every time the configuration file is modified and saved.
