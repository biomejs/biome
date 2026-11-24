---
"@biomejs/biome": patch
---

Added the new nursery rule [`useTanstackQueryExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-tanstack-query-exhaustive-dependencies/), which reports TanStack Query options whose `queryKey` doesn't include every value that `queryFn` uses. When the key misses a value, changing that value doesn't fetch new data, and the query keeps returning data for the old value.

```js
function Todo({ todoId }) {
  useQuery({
    queryKey: ["todo"],
    queryFn: () => fetchTodo(todoId),
  });
}
```
