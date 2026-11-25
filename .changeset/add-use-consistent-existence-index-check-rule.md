---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentExistenceIndexCheck`](https://biomejs.dev/linter/rules/use-consistent-existence-index-check/), which reports checks such as `index < 0`, `index >= 0`, and `index > -1` on the result of `indexOf()`, `lastIndexOf()`, `findIndex()`, or `findLastIndex()`, and suggests comparing with `-1` instead.

```js
const index = list.indexOf(item);
if (index < 0) {}
```
