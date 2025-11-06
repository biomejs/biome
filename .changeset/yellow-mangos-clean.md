---
"@biomejs/biome": patch
---

Fixed an edge case in the [`useArrowFunction`](https://biomejs.dev/linter/rules/use-arrow-function/) rule.

The rule no longer emits diagnostics for or offers to fix functions that reference
the [arguments object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/arguments),
because that object is undefined for arrow functions.

#### Valid:

```ts
// Valid: this function cannot be transformed into an arrow function because
// arguments is not defined for arrow functions.
const getFirstArg = function () {
  return arguments[0];
}
```
