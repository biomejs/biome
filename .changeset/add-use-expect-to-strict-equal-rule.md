---
"@biomejs/biome": patch
---

Added the nursery rule [`useExpectToStrictEqual`](https://biomejs.dev/linter/rules/use-expect-to-strict-equal/), which reports tests that compare values with `toEqual()`, and suggests using `toStrictEqual()` instead. `toEqual()` ignores properties set to `undefined`, so the following test passes even though the two objects are different.

```js
expect({ a: "a", b: undefined }).toEqual({ a: "a" });
```
