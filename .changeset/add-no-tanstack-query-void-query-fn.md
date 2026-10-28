---
"@biomejs/biome": patch
---

Added the nursery rule [`noTanstackQueryVoidQueryFn`](https://biomejs.dev/linter/rules/no-tanstack-query-void-query-fn/), which disallows TanStack Query functions that don't return any data. TanStack Query doesn't accept `undefined` as the data of a query, so the query fails with an error.

```js
import { useQuery } from "@tanstack/react-query";

useQuery({
    queryKey: ["todos"],
    queryFn: async () => {
        await fetch("/api/todos");
    },
});
```

The rule belongs to the `tanstackQuery` and `types` domains.
