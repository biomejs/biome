---
"@biomejs/biome": patch
---

Added new nursery rule [`noDeprecatedImports`](https://biomejs.dev/linter/rules/no-deprecated-imports/) to flag imports of deprecated symbols.

#### Invalid example

```js
// foo.js
import { oldUtility } from "./utils.js";
```

```js
// utils.js
/**
 * @deprecated
 */
export function oldUtility() {}
```

#### Valid examples

```js
// foo.js
import { newUtility, oldUtility } from "./utils.js";
```

```js
// utils.js
export function newUtility() {}
    
// @deprecated (this is not a JSDoc comment)
export function oldUtility() {}
```
