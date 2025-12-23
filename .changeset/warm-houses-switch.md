---
"@biomejs/biome": patch
---

Fixed [#6783](https://github.com/biomejs/biome/issues/6783): now, when a path is provided via `--stdin-file-path`, Biome checks whether the file exists on disk. If the path doesn't exist, the `files.includes` check is skipped, but VCS ignore rules still apply.

