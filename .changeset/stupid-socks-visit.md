---
"@biomejs/biome": patch
---

Fixed an issue where `textDocument/codeAction` in the LSP could respond with outdated text edits after the workspace watcher observed outdated changes to the file.
