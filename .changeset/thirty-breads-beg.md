---
"@biomejs/biome": patch
---

Fixed `--diagnostic-level` not fully filtering diagnostics. Setting `--diagnostic-level=error` now correctly excludes warnings and infos from both the output and the summary counts.
