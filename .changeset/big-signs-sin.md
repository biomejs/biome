---
"@biomejs/biome": patch
---

Fixed [#12184](https://github.com/biomejs/biome/issues/12184): [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) no longer misses relative imports with `.jsx` or `.tsx` extensions when using `extensionMappings` or `forceJsExtensions`. The rule reports these imports when the configured replacement resolves to an existing file.
