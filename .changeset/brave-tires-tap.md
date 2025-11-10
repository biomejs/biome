---
"@biomejs/biome": patch
---

Fixed a bug where the Biome Language Server would enable its project file watcher even when no project rules were enabled.

Now the watching of nested configuration files and nested ignore files is delegated to the editor, if their LSP spec supports it.
