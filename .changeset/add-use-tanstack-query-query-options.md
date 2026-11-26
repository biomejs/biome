---
"@biomejs/biome": patch
---

Added the nursery rule [`useTanstackQueryQueryOptions`](https://biomejs.dev/linter/rules/use-tanstack-query-query-options/), which reports TanStack Query options that set `queryKey` or `queryFn` where the query is used, and query keys that are written by hand in query client methods. Creating the options once with `queryOptions()` keeps the key and the function of a query together.

```js
import { useQuery, useQueryClient } from "@tanstack/react-query";

function Todo({ id }) {
    const query = useQuery({ queryKey: ["todo", id], queryFn: () => fetchTodo(id) });

    const queryClient = useQueryClient();
    const refresh = () => queryClient.invalidateQueries({ queryKey: ["todo", id] });
}
```
