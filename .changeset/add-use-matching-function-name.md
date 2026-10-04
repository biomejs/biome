---
"@biomejs/biome": patch
---

Added the new nursery rule [`useMatchingFunctionName`](https://biomejs.dev/linter/rules/use-matching-function-name/), which requires a named function expression to have the same name as the variable or property it's assigned to.

```js
// The function name `bar` doesn't match the variable name `foo`.
const foo = function bar() {};
```
