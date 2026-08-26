---
"@biomejs/biome": patch
---

Added the nursery rule [`useFlatMathMinMax`](https://biomejs.dev/linter/rules/use-flat-math-min-max/). Because `Math.min()` and `Math.max()` accept any number of arguments, the rule reports unnecessary nested calls to the same method:

```js
Math.max(Math.max(a, b), c);
```

The fix flattens this expression to `Math.max(a, b, c)`.
