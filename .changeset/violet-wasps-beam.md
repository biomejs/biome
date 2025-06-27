---
"@biomejs/biome": patch
---

Reduced the strictness of Biome in `stdin` mode when `--stdin-file-path` doesn't contain any extension. Now Biome doesn't exist with an error core, and returns the original content.
