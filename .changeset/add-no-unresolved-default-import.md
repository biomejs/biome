---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUnresolvedDefaultImport`](https://biomejs.dev/linter/rules/no-unresolved-default-import/), which reports default imports from modules that have no default export. The rule is part of the `project` domain.

```js
// foo.js only contains `export function foo() {}`
import foo from "./foo.js";
```
