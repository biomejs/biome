---
"@biomejs/biome": patch
---

Added the lint rule [`noNestedPromises`](https://biomejs.dev/linter/rules/no-nested-promises/). This rule detects nested `.then()` or `.catch()` calls that could be refactored into flat promise chains.

```js
// Invalid: nested promise that can be flattened
doThing().then(function() {
  return doOtherThing().then(console.log);
});

// Valid: flat promise chain
doThing()
  .then(() => doOtherThing())
  .then(console.log);
```

The rule intelligently allows nesting when the inner callback references variables from the outer scope, as these cases cannot be safely flattened.
