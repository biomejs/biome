---
"@biomejs/biome": patch
---

Added the nursery rule [`noNegatedArrayPredicate`](https://biomejs.dev/linter/rules/no-negated-array-predicate/), which reports negated calls to `Array#some()` and `Array#every()` and suggests moving the negation into the callback function.

```js
// Invalid: use `array.every((item) => !isUnicorn(item))` instead.
const isMissing = !array.some((item) => isUnicorn(item));
```
