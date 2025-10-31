---
"@biomejs/biome": patch
---

Added the rule [`noParametersOnlyUsedInRecursion`](https://biomejs.dev/linter/rules/no-parameters-only-used-in-recursion/).

This rule detects function parameters that are exclusively used in recursive calls and can be removed to simplify the function signature since they are effectively unused.

```js
function factorial(n, acc) {
    if (n === 0) return 1;
    return factorial(n - 1, acc);  // acc is only used here
}
```


Fixes [#6484](https://github.com/biomejs/biome/issues/6484).
