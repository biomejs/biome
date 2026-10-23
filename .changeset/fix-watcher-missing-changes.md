---
"@biomejs/biome": patch
---

Fixed the file watcher of the Biome daemon and language server missing file changes. The watcher could stop for good when it read the workspace while another update was being applied, and it could miss changes made right after a project scan.
