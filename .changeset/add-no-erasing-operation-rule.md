---
"@biomejs/biome": patch
---

Added the nursery rule [`noErasingOperation`](https://biomejs.dev/linter/rules/no-erasing-operation/), which disallows operations with `0` that always result in zero: multiplying by `0`, a bitwise AND with `0`, and dividing `0` by a number. The other value in these operations has no effect on the result, which usually means the code has a mistake.

```js
const total = price * 0;
```
