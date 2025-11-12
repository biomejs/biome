---
"@biomejs/biome": patch
---

Fixed [#8004](https://github.com/biomejs/biome/issues/8004): [`noParametersOnlyUsedInRecursion`](https://biomejs.dev/linter/rules/no-parameters-only-used-in-recursion/) now correctly detects recursion by comparing function bindings instead of just names.

Previously, the rule incorrectly flagged parameters when a method had the same name as an outer function but called the outer function (not itself):

```js
function notRecursive(arg) {
  return arg;
}

const obj = {
  notRecursive(arg) {
    return notRecursive(arg); // This calls the outer function, not the method itself
  },
};
```

Biome now properly distinguishes between these cases and will not report false positives.
