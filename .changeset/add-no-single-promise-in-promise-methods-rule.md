---
"@biomejs/biome": patch
---

Added the new nursery rule [`noSinglePromiseInPromiseMethods`](https://biomejs.dev/linter/rules/no-single-promise-in-promise-methods/), which reports an array with a single element passed to `Promise.all()`, `Promise.any()`, or `Promise.race()`. The rule offers a fix that awaits the value directly or uses `Promise.resolve()`.

```js
const result = await Promise.all([promise]);
```
