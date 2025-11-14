---
"@biomejs/biome": patch
---

Added the nursery rule [`useFind`](https://biomejs.dev/linter/rules/use-find/). Enforce the use of Array.prototype.find() over Array.prototype.filter() followed by [0] when looking for a single result.

**Invalid:**

```js
[1, 2, 3].filter(x => x > 1)[0];

[1, 2, 3].filter(x => x > 1).at(0);
```
