---
"@biomejs/biome": patch
---

Improved the performance of `biome lint`, `biome check`, and `biome ci` on Linux. Biome no longer reads the CPU limits of its control group for every file it analyzes. Each read took dozens of system calls.
