---
"@biomejs/biome": patch
---

Fixed [#11817](https://github.com/biomejs/biome/issues/11817): Biome no longer crashes when its output is piped to a program that exits early, such as `head`. Output to the closed pipe is now discarded and Biome exits normally.
