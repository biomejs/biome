---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexQueryFilter`](https://biomejs.dev/linter/rules/no-convex-query-filter/), which reports `.filter()` calls on Convex database queries. The database reads every document in the query before `.filter()` drops the ones that don't match, so `.withIndex()` or filtering the collected results in code is preferred. The rule belongs to the `convex` domain.

```js
const messages = await ctx.db
    .query("messages")
    .filter((q) => q.eq(q.field("author"), args.author))
    .collect();
```
