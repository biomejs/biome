---
"@biomejs/biome": patch
---

Added the nursery rule [`useConvexExplicitTableIds`](https://biomejs.dev/linter/rules/use-convex-explicit-table-ids/), which reports Convex `db.get()`, `db.patch()`, `db.replace()`, and `db.delete()` calls that don't pass the table name as the first argument.

```js
const message = await ctx.db.get(messageId);
```
