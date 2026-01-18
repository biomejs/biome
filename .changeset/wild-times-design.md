---
"@biomejs/biome": patch
---

Added the nursery rule [`noFloatingClasses`](https://biomejs.dev/linter/rules/no-floating-classes). Disallow `new` operators outside of assignments or comparisons.

**Invalid:**

```js
new Date();
```
