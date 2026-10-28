---
"@biomejs/biome": patch
---

Fixed an issue where the CLI, when it ran with `--use-server` alongside an editor connected to the same daemon, checked the unsaved text of the files open in the editor instead of the files on disk, and could make the editor's text get lost. The CLI now checks the files on disk, and every editor and CLI run connected to the daemon keeps its own text of the files it opens.
