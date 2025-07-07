---
"@biomejs/biome": minor
---

Fixed [#6646](https://github.com/biomejs/biome/issues/6646): `.gitignore` files
are now picked up even when running Biome from a nested directory, or when the
ignore file itself is ignored through `files.includes`.
