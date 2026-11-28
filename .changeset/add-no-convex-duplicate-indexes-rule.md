---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexDuplicateIndexes`](https://biomejs.dev/linter/rules/no-convex-duplicate-indexes/), which reports Convex database indexes whose fields are the first fields of another index on the same table. The rule is part of the `convex` domain.

```js
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", ["author"]) // reported
	.index("by_author_and_channel", ["author", "channel"]);
```
