---
"@biomejs/biome": patch
---

Fixed the LSP method `workspace/didChangeWorkspaceFolders` to perform incremental updates instead of replacing the entire folder list.
