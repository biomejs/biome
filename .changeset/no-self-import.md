---
"@biomejs/biome": patch
---

Added the new nursery rule [`noSelfImport`](https://biomejs.dev/linter/rules/no-self-import/), which forbids a module from importing itself.

```js
// foo.js
import foo from "./foo.js";
```
