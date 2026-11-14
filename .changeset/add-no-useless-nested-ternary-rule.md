---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessNestedTernary`](https://biomejs.dev/linter/rules/no-useless-nested-ternary/), which reports nested ternary expressions that repeat a result of the expression around them. The conditions of such expressions can be combined with `&&` or `||`.

```js
const result = a ? (b ? c : d) : d;
```
