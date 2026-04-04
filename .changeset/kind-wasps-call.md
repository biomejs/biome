---
"@biomejs/biome": patch
---

Fixed [#9741](https://github.com/biomejs/biome/issues/9741): the LSP server now correctly returns the [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) code action when the client requests it via `source.organizeImports.biome` in the `only` filter. Previously, editors with `codeAction/resolve` support (e.g. Zed) received an empty response because the action was serialized with the wrong kind (`source.biome.organizeImports` instead of `source.organizeImports.biome`).
