---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessLengthCheck`](https://biomejs.dev/linter/rules/no-useless-length-check/), which reports array length checks that don't change the result: a check that the array is empty next to `every()`, a check that it isn't empty next to `some()`, or a check that it isn't empty around a `for...of` loop over the same array.

```js
if (array.length > 0 && array.some(Boolean)) {}
```
