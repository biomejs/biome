---
"@biomejs/biome": patch
---

Added the nursery rule [`useValidNamespaceImports`](https://biomejs.dev/linter/rules/use-valid-namespace-imports/), which reports names read from a namespace import that the imported module doesn't export. It also reports assignments to members of a namespace import, and namespace imports of modules that have no exports.

```js
import * as utils from "./utils.js";

utils.subtract(2, 1); // `./utils.js` doesn't export `subtract`
```
