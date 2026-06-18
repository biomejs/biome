---
"@biomejs/biome": patch
---

Fixed a bug where the Biome Daemon didn't correctly shutdown when closing the editor, the daemon was in the middle of an operation. This bug was more evident for operations such as scanning.
