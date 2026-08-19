---
"@biomejs/biome": patch
---

Fixed a memory leak in the LSP server where memory usage kept growing over long editor sessions. Closing a file or a project now correctly frees the memory it used.
