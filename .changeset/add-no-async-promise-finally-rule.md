---
"@biomejs/biome": patch
---

Added the nursery rule [`noAsyncPromiseFinally`](https://biomejs.dev/linter/rules/no-async-promise-finally/), which reports async functions passed to a promise's `.finally()` method. If such a function fails, its error replaces the original result or error of the promise.

```js
promise.finally(async () => {
    await cleanup();
});
```
