---
"@biomejs/biome": patch
---

Fixed [#12098](https://github.com/biomejs/biome/issues/12098): [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) now reports relative imports without extensions in Vue, Svelte, and Astro files when `html.experimentalFullSupportEnabled` is enabled.
