---
"@biomejs/biome": patch
---
Fixed [#9941](https://github.com/biomejs/biome/issues/9941): Biome now exits with a non-zero code when a file exceeds `files.maxSize` and `--error-on-warnings` is enabled.