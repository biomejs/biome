---
"@biomejs/biome": patch
---

Fixed [#9557](https://github.com/biomejs/biome/pull/9557): LSP server crash when the client does not send `workspaceFolders` in `InitializeParams`. The file watcher registration now uses the original `rootUri` directly instead of round-tripping through a filesystem path.
