---
"@biomejs/biome": patch
---

Improved the performance of `biome format --write` for files without syntax errors. Biome no longer parses the formatted code of these files a second time. JSON files are not affected.
