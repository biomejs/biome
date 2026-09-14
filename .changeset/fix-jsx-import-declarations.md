---
"@biomejs/biome": patch
---

Fixed [#8893](https://github.com/biomejs/biome/issues/8893): [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) no longer suggests adding `.ts` to `.jsx` imports when a colocated `.d.ts` file provides type declarations.
