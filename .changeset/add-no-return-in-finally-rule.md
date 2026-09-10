---
"@biomejs/biome": patch
---

Added the nursery rule [`noReturnInFinally`](https://biomejs.dev/linter/rules/no-return-in-finally/). This rule disallows return statements in `Promise.prototype.finally()` callbacks, including inside nested blocks and conditional branches. Returns in nested functions are ignored by the rule.

```js
// Invalid: return in finally callback
Promise.resolve(1).finally(() => { return 2 })

// Valid: no return in finally callback
Promise.resolve(1).finally(() => { console.log(2) })
```

Returning a value from a `Promise.prototype.finally()` callback does not replace the original promise's fulfillment value, which can be confusing. Returned promises and thenables are awaited, and their rejection rejects the resulting promise.
