---
"@biomejs/biome": patch
---

Fixed [#12796](https://github.com/biomejs/biome/issues/12796): [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/) no longer reports ternaries whose operand is a bigint expression, such as `-1n` or `2n ** 62n - 1n`, or a variable or parameter default initialized with one. The unsafe fix turned these into `Math.min()`/`Math.max()` calls that throw a `TypeError`.

```js
const MAX = 2n ** 62n - 1n;
const clamp = (x) => (x > MAX ? MAX : x);
```
