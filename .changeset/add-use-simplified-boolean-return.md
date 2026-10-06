---
"@biomejs/biome": patch
---

Added the new nursery rule [`useSimplifiedBooleanReturn`](https://biomejs.dev/linter/rules/use-simplified-boolean-return/), which reports `if` statements that only return `true` or `false` and can be replaced by returning the condition directly.

```js
function isPositive(value) {
  if (value > 0) {
    return true;
  }
  return false;
}
```
