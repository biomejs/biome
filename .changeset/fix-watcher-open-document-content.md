---
"@biomejs/biome": patch
---

Fixed the language server losing track of an open file when the file watcher re-indexed it. Biome now keeps the editor's text of open files, and diagnostics and code actions stay at the right positions.
