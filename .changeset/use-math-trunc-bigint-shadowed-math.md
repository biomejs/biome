---
"@biomejs/biome": patch
---

Fixed [#12872](https://github.com/biomejs/biome/issues/12872): [`useMathTrunc`](https://biomejs.dev/linter/rules/use-math-trunc/) no longer reports `~~x` or `parseInt(String(x), 10)` when `x` is a BigInt, because `Math.trunc()` throws a `TypeError` for a BigInt. This covers BigInt literals such as `10n`, `BigInt()` calls, and variables or parameters annotated as `bigint` or initialized with a BigInt. The rule also no longer reports code where a local `Math` declaration would replace the built-in object in the fix.

```js
~~10n; // `Math.trunc(10n)` throws
function compute(x, Math) {
  return x | 0; // `Math` here isn't the built-in object
}
```
