---
"@biomejs/biome": minor
---

Added the rule [`noParametersOnlyUsedInRecursion`](https://biomejs.dev/linter/rules/no-parameters-only-used-in-recursion/).

This rule detects function parameters that are exclusively used in recursive calls and can be removed to simplify the function signature.

```js
function factorial(n, acc) {
    if (n === 0) return 1;
    return factorial(n - 1, acc);  // acc is only used here
}
```

Biome now reports parameters that are only passed to recursive calls, as they are effectively unused. The rule provides an unsafe code action to prefix the parameter with an underscore if the behavior is intentional.

Fixes [#6484](https://github.com/biomejs/biome/issues/6484).
