---
"@biomejs/biome": patch
---

Added the nursery rule [`useConvexAccessControl`](https://biomejs.dev/linter/rules/use-convex-access-control/), which reports exported Convex queries, mutations and actions whose handler doesn't call an access control function, such as `requireUser(ctx)`. The rule belongs to the `convex` domain.

```js
import { mutation } from "./_generated/server";

export const send = mutation({
    args: {},
    handler: async (ctx) => {
        await ctx.db.insert("messages", { body: "hello" });
    },
});
```
