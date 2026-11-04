---
"@biomejs/biome": patch
---

Added the nursery rule [`noDynamicRequire`](https://biomejs.dev/linter/rules/no-dynamic-require/), which disallows `require()` calls whose module name is built at runtime. When the `checkDynamicImports` option is enabled, the rule also checks `import()` calls.

```js
require("./locales/" + language);
```
