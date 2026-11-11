---
"@biomejs/biome": patch
---

Added the nursery rule [`useTopLevelRequire`](https://biomejs.dev/linter/rules/use-top-level-require/), which reports `require()` calls that aren't at the top level of the module, such as calls inside functions, blocks, or `try` statements.

```js
function readFile(filename, callback) {
    const fs = require("fs");
    fs.readFile(filename, callback);
}
```
