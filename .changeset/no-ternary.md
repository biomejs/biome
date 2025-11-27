---
"@biomejs/biome": patch
---

Added the nursery rule [`noTernary`](https://biomejs.dev/linter/rules/no-ternary/). Disallow ternary operators.

**Invalid:**

```js
const foo = isBar ? baz : qux;
```
