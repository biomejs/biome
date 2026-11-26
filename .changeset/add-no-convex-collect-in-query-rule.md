---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexCollectInQuery`](https://biomejs.dev/linter/rules/no-convex-collect-in-query/), which reports Convex database queries that load every matching document with `.collect()`. Use `.take()` or `.paginate()` instead.

```js
const messages = await ctx.db.query("messages").collect();
```
