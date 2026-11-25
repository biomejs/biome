---
"@biomejs/biome": patch
---

Improved the performance of commands that process files, such as `biome check`, `biome lint` and `biome format`. Biome now starts processing files while it is still reading the folders of the project, and it no longer checks again whether the folders that contain a file are ignored. On projects with a few thousand files, `biome check` was between 14% and 18% faster in our measurements.
