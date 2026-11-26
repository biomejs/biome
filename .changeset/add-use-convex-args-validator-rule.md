---
"@biomejs/biome": patch
---

Added the nursery rule [`useConvexArgsValidator`](https://biomejs.dev/linter/rules/use-convex-args-validator/), which requires Convex queries, mutations, and actions to define an `args` validator, so that Convex checks the arguments of every call before the handler runs.

```js
export const list = query({
  handler: async (ctx) => {
    return await ctx.db.query("tasks").collect();
  },
});
```
