---
"@biomejs/biome": patch
---

Added the nursery rule [`useTanstackQueryStableQueryClient`](https://biomejs.dev/linter/rules/use-tanstack-query-stable-query-client/), which reports a TanStack Query `QueryClient` created inside a React component or custom hook. Such a client is created again on every render, and its cache starts empty each time.

```jsx
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

function App() {
    const queryClient = new QueryClient();
    return <QueryClientProvider client={queryClient}><Home /></QueryClientProvider>;
}
```
