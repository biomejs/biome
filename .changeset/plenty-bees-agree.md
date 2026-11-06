---
"@biomejs/biome": patch
---

Fixed an issue where Biome could silently skip configured plugins. Biome now reports an error when those plugins are unavailable during analysis.

