---
"@biomejs/biome": patch
---

Added the nursery rule [`noConstantComparisons`](https://biomejs.dev/linter/rules/no-constant-comparisons/), which reports `&&` and `||` conditions that can never be true, are always true, or contain a check with no effect.

```js
if (statusCode <= 400 && statusCode > 500) {}
```
