---
"@biomejs/biome": patch
---

Added the nursery rule [`useCatchErrorName`](https://biomejs.dev/linter/rules/use-catch-error-name/), which requires a specific name for caught errors in `try...catch` statements and in the error handlers of `promise.catch()` and `promise.then()`. The name defaults to `error` and can be changed with the `name` option.

```js
try {} catch (err) {} // `err` should be named `error`
```
