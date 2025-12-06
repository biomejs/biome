---
"@biomejs/biome": patch
---

Fixed [#6569](https://github.com/biomejs/biome/issues/6569): Allow files to export from themselves with `noImportCycles`.

This means the following is now allowed:

```js
// example.js
export function example() {
  return 1;
}

// Re-exports all named exports from the current module under a single namespace
// and then imports the namespace from the current module.
// Allows for encapsulating functions/variables into a namespace instead
// of using a static class.
export * as Example from './example.js';

import { Example } from './example.js';
```
