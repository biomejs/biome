---
"@biomejs/biome": patch
---

Added the nursery rule [`noPathConcat`](https://biomejs.dev/linter/rules/no-path-concat/), which disallows building file paths and URLs by joining strings to `__dirname`, `__filename`, `import.meta.dirname`, `import.meta.filename`, or `import.meta.url`.

```js
const fullPath = __dirname + "/foo.js"; // reported
```
