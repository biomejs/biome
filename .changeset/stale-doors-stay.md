---
"@biomejs/biome": patch
---

Fixed [#10493](https://github.com/biomejs/biome/issues/10493): [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now correctly separates types from a default named import when all imports are types and the `style` option is set to `separatedType`.
