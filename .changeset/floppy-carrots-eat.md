---
"@biomejs/biome": patch
---

Added the nursery rule [useConsistentFunctionStyle](https://biomejs.dev/linter/rules/use-consistent-function-style/), which requires a consistent style for defining functions.

By default, the rule reports the following declaration because it requires a function expression assigned to a variable:

```js
function greet() {
    return "Hello";
}
```
