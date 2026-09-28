---
"@biomejs/biome": patch
---

Fixed [#11881](https://github.com/biomejs/biome/issues/11881): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) no longer reports imports of a package that ships its own types when a deprecated `@types` stub for it is also installed.
