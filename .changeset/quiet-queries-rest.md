---
"@biomejs/biome": patch
---

Added the nursery rule [`noTanstackQueryRestDestructuring`](https://biomejs.dev/linter/rules/no-tanstack-query-rest-destructuring/). The rule disallows rest destructuring on the result of TanStack Query hooks, which subscribes the component to every field of the result.

```js
import { useQuery } from "@tanstack/react-query";

const { data, ...rest } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
```
