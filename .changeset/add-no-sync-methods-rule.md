---
"@biomejs/biome": patch
---

Added the new nursery rule [`noSyncMethods`](https://biomejs.dev/linter/rules/no-sync-methods/), which reports calls to functions whose names end with `Sync`, such as `fs.readFileSync()`. These calls block the program until they finish. The `allowAtRootLevel` option allows them outside of functions, and the `ignore` option allows specific function names.

```js
const contents = fs.readFileSync("data.txt");
```
