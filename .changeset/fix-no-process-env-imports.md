---
"@biomejs/biome": patch
---

Fixed [#9061](https://github.com/biomejs/biome/issues/9061): `noProcessEnv` now also detects `process.env` when `process` is imported from the `"process"` or `"node:process"` modules.

Previously, only the global `process` object was flagged:

```js
import process from "node:process";
// This was not flagged, but now it is:
console.log(process.env.NODE_ENV);
```
