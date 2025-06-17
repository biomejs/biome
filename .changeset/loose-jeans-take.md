---
"@biomejs/biome": patch
---

Fixed a bug where passing `--max-diagnostics=0` would return a zero code even when errors were emitted.
