---
"@biomejs/biome": patch
---

Fixed [#9557](https://github.com/biomejs/biome/pull/9557): Biome's LSP server no longer crashes on startup when used with editors that don't send `workspaceFolders` during initialization. This affected any LSP client that only sends `rootUri`, which is valid per the LSP specification.
