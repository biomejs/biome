---
"@biomejs/biome": patch
---

Fixed `noProcessEnv` to also detect `process.env` when `process` is imported from the `"process"` or `"node:process"` modules.

Previously, only the global `process` object was flagged:

```js
import process from "node:process";
// This was not flagged, but now it is:
console.log(process.env.NODE_ENV);
```
