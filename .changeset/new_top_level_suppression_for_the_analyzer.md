---
cli: minor
---

# New top-level suppression for the analyzer

The Biome analyzer now supports a new top-level suppression. These suppression have to be placed at the top of the file, and they **must** be multiple line comments.

The analyzer rules specified inside the block comment will be suppressed for the whole file.

In the example, we suppress the rules `lint/style/useConst` and `lint/suspicious/noDebugger` for the whole file:

```js
// main.js
/**
 * biome-ignore lint/style/useConst: i like let
 * biome-ignore lint/suspicious/noDebugger: needed now
 */

let path = "/path";
let _tmp = undefined;
debugger
```
