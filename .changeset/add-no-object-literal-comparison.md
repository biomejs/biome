---
"@biomejs/biome": patch
---

Added the new nursery rule [`noObjectLiteralComparison`](https://biomejs.dev/linter/rules/no-object-literal-comparison/), which reports comparisons with an empty object `{}` or an empty array `[]`. These comparisons never check whether a value is empty, because objects and arrays are compared by reference.

```js
if (value === {}) {}
```
