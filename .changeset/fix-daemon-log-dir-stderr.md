---
"@biomejs/biome": patch
---

Fixed [#11025](https://github.com/biomejs/biome/issues/11025): the Biome daemon no longer prints `Error reading the log directory/files: No such file or directory` to stderr on its first run. The log directory is now created before the log file appender starts, so an editor connecting to a freshly installed Biome no longer reports an error from the language server.
