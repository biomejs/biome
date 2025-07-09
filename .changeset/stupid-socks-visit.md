---
"@biomejs/biome": patch
---

Fixed that `textDocument/codeAction` in LSP could response outdated text edits after the workspace watcher observed outdated changes to the file.
