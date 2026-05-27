---
"@biomejs/biome": patch
---

Fixed [#8590](https://github.com/biomejs/biome/issues/8590): improved [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/), [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/), [`noUnusedFunctionParameters`](https://biomejs.dev/linter/rules/no-unused-function-parameters/), and [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) for Svelte, Vue, and Astro files (with `html.experimentalFullSupportEnabled`). Bindings used only in the template — including component tags, attribute interpolations, directives, `bind:` shorthand, and snippet parameters — are no longer reported as unused, while genuinely unused ones still are.
