---
"@biomejs/biome": patch
---

Fixed a bug where closing one editor stopped a shared Biome daemon used by other editors. LSP proxy processes now exit when either the editor or daemon disconnects.
