---
"@biomejs/biome": patch
---

Added the new nursery rule [`noMultiAssign`](https://biomejs.dev/linter/rules/no-multi-assign). This rule helps to prevent multiple chained assignments.

For example, the following code triggers because there are two assignment expressions in the same statement.

```js
const a = (b = 0);
```
