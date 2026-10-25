---
"@biomejs/biome": patch
---

Improved the performance of `biome lint`, `biome check`, and `biome ci` on Linux. Biome now spends less time in the operating system for each file it analyzes, so linting large projects is faster.
