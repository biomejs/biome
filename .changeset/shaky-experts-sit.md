---
"@biomejs/biome": patch
---

Fixed [#6799](https://github.com/biomejs/biome/issues/6799): The [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles/) rule now ignores type-only imports if the new `ignoreTypes` option is enabled (enabled by default).
