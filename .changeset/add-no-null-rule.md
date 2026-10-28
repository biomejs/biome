---
"@biomejs/biome": patch
---

Added the nursery rule [`noNull`](https://biomejs.dev/linter/rules/no-null/), which reports uses of the `null` literal and suggests `undefined` instead. Strict comparisons such as `foo === null` are allowed.

```js
let foo = null;
```
