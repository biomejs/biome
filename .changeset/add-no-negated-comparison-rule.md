---
"@biomejs/biome": patch
---

Added the nursery rule [`noNegatedComparison`](https://biomejs.dev/linter/rules/no-negated-comparison/), which reports equality comparisons that are negated as a whole and suggests the opposite operator instead.

```js
const isDifferent = !(a === b); // use `a !== b` instead
```
