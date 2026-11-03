---
"@biomejs/biome": patch
---

Fixed a slowdown in `biome lint`, `biome check` and `biome format` that grew with the number of files: looking up an open file took longer after every file Biome had already processed. Large projects are processed faster again.
