---
"@biomejs/biome": patch
---

Added the nursery rule [`noReturnInFinally`](https://biomejs.dev/linter/rules/no-return-in-finally/). This rule disallows return statements in `finally()` promise callbacks.

```js
// Invalid: return in finally callback
Promise.resolve(1).finally(() => { return 2 })

// Valid: no return in finally callback
Promise.resolve(1).finally(() => { console.log(2) })
```

The return value from a `finally()` callback is ignored, making any return statement potentially confusing and indicating a likely mistake in the code.
