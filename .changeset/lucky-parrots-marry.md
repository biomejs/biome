---
"@biomejs/biome": patch
---

The Biome LSP server no longer responds with an error for a
`textDocument/codeActions` request when Biome doesn't support a feature for the
file (e.g. Code actions aren't supported in GritQL files).
