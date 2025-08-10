---
"@biomejs/biome": patch
---

The [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) rule now correctly detects imports with an invalid extension. For example, importing `.ts` file with `.js` extension is flagged by default. If you are using TypeScript with neither the `allowImportingTsExtensions` option nor the `rewriteRelativeImportExtensions` option, it's recommended to turn on the `forceJsExtensions` option of the rule.
