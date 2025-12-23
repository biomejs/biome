---
"@biomejs/biome": patch
---

Fixed [#6783](https://github.com/biomejs/biome/issues/6783): now, when a path is provided via `--stdin-file-path`, Biome checks whether the file exists within the current project. If the path doesn't exist, ignore checks - `files.includes` and VCS-ignore files - are skipped, and the file is handled.

