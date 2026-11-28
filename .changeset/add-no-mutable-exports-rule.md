---
"@biomejs/biome": patch
---

Added the nursery rule [`noMutableExports`](https://biomejs.dev/linter/rules/no-mutable-exports/). It reports exported variables declared with `let` or `var`, which other modules see change whenever they are reassigned.

```js
export let count = 1;
```
