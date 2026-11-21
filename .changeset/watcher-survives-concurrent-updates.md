---
"@biomejs/biome": patch
---

Fixed the file watcher of the Biome language server stopping when a file changed on disk while the editor was updating another file. Afterward, Biome missed every change on disk until it restarted, so diagnostics that depend on other files, such as those of the Tailwind CSS rules and of the project rules, became stale.
