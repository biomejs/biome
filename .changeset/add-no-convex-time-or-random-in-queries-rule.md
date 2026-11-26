---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexTimeOrRandomInQueries`](https://biomejs.dev/linter/rules/no-convex-time-or-random-in-queries/), which reports Convex queries that read the current time or generate random values. Convex caches query results, so these values can be outdated or stay the same between calls.

```js
export const pickWinner = query({
  handler: async (ctx) => {
    const entries = await ctx.db.query("entries").collect();
    return entries[Math.floor(Math.random() * entries.length)];
  },
});
```
