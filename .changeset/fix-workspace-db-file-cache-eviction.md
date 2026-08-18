---
"@biomejs/biome": patch
---

Fixed a memory leak in the LSP server where the cached parsed source of a file was never evicted after the file or its project was closed. Closing a file or a project now also removes its cached parsed sources, preventing unbounded memory growth in long-running editor sessions.
