---
"@biomejs/biome": patch
---

Added the new nursery rule [`noArrayMethodsOnArguments`](https://biomejs.dev/linter/rules/no-array-methods-on-arguments/). It reports calls to array methods on the `arguments` object, which isn't an array, so these calls throw a `TypeError` when the code runs.

```js
function sum() {
  return arguments.reduce((total, value) => total + value, 0);
}
```
