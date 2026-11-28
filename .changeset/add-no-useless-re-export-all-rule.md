---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessReExportAll`](https://biomejs.dev/linter/rules/no-useless-re-export-all/). It reports `export * from` a module that has no named exports, which exports nothing. The rule belongs to the `project` domain.

```js
// foo.js only has a default export.
export * from "./foo.js";
```
