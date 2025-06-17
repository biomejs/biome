---
"@biomejs/biome": patch
---

Fixed a bug where Biome didn't report any error when `--stdin-file-path` didn't have any extension.
Now Biome returns an error if `--stdin-file-path` doesn't have an extension.
