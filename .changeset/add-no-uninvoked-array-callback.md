---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUninvokedArrayCallback`](https://biomejs.dev/linter/rules/no-uninvoked-array-callback/), which reports array methods such as `map()` and `forEach()` called with a callback on an array that only has empty slots. The callback never runs in this case.

```js
const list = new Array(5).map(() => createElement());
```
