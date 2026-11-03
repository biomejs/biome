---
"@biomejs/biome": patch
---

Added the nursery rule [`useTanstackQueryInfiniteQueryPropertyOrder`](https://biomejs.dev/linter/rules/use-tanstack-query-infinite-query-property-order/), which enforces the order `queryFn`, `getPreviousPageParam`, `getNextPageParam` in the options of TanStack Query infinite queries. TanStack Query infers types from this order.

```js
import { useInfiniteQuery } from "@tanstack/react-query";

useInfiniteQuery({
  getNextPageParam: (lastPage) => lastPage.nextId,
  queryFn: ({ pageParam }) => fetchProjects(pageParam),
});
```
