---
"@biomejs/biome": minor
---

Added new lint rule [`noShadow`](http://biome.dev/linter/rules/no-shadow), a port of eslint's `no-shadow`.

This rule disallows variable declarations from shadowing variables declared in an outer scope. For example:

```js
const foo = 1;

function bar() {
  const foo = 2; // This variable shadows the outer foo
}
```
