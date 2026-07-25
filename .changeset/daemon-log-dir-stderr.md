---
"@biomejs/biome": patch
---

Fixed a spurious `Error reading the log directory/files: No such file or directory` message printed to stderr the first time the daemon starts. The log directory is now created before the rolling file appender inspects it, so editors no longer surface an error when they connect to the LSP for the first time.
