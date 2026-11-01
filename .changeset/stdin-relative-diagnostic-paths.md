---
"@biomejs/biome": patch
---

Fixed diagnostics for `--stdin-file-path` showing the full absolute path.
Biome now shows the path relative to your working directory, same as when it reads files from disk, so `/home/user/project/src/foo.js:3:1` becomes `src/foo.js:3:1`.
