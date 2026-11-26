---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexOldRegisteredFunctionSyntax`](https://biomejs.dev/linter/rules/no-convex-old-registered-function-syntax/), which reports Convex queries, mutations, and actions registered with a plain function instead of an object. The object syntax lets you declare an `args` validator.

```js
export const list = query(async (ctx) => {
  return await ctx.db.query("messages").collect();
});
```
