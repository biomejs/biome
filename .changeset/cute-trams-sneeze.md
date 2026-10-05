---
"@biomejs/biome": patch
---

Fixed [#7354](https://github.com/biomejs/biome/issues/7354): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) accepts untyped JavaScript package entrypoints, including legacy `main` paths without extensions or pointing to directories. Type-only imports still require declarations, and indexed ES modules retain diagnostics for missing exports.
