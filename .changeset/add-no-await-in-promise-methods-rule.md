---
"@biomejs/biome": patch
---

Added the new nursery rule [`noAwaitInPromiseMethods`](https://biomejs.dev/linter/rules/no-await-in-promise-methods/), which reports `await` inside the array passed to `Promise.all()`, `Promise.allSettled()`, `Promise.any()`, or `Promise.race()`. The `await` pauses the code before the method is called, so the promises no longer run at the same time.

```js
Promise.all([await promise, anotherPromise]);
```
