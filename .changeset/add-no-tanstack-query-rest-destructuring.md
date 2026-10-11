---
"@biomejs/biome": patch
---

Added the nursery rule [`noTanstackQueryRestDestructuring`](https://biomejs.dev/linter/rules/no-tanstack-query-rest-destructuring/), which disallows rest destructuring and object spread on TanStack Query results. Both read every property of the result, so the component updates on every change to the query.

```js
import { useQuery } from "@tanstack/react-query";

const { data, ...rest } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
```

The rule belongs to the new `tanstackQuery` domain.
