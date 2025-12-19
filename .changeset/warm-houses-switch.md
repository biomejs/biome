---
"@biomejs/biome": patch
---

Fixed [#6783](https://github.com/biomejs/biome/issues/6783): stdin formatting using `--stdin-file-path` is no longer blocked by `files.includes` when the provided path doesn't exist on disk.

