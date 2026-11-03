---
"@biomejs/biome": patch
---

Added the new nursery rule [`noNumberArgumentOutOfRange`](https://biomejs.dev/linter/rules/no-number-argument-out-of-range/), which reports number literal arguments of `toString()`, `toFixed()`, `toExponential()` and `toPrecision()` that are outside the range the method accepts. These calls throw a `RangeError` at runtime.

```js
value.toString(64);
```
