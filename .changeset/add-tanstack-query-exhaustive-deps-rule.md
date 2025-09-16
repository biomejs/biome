---
"@biomejs/biome": minor
---

Added the rule [`useTanStackQueryExhaustiveDeps`](https://biomejs.dev/linter/rules/use-tanstack-query-exhaustive-deps/).

This rule ensures that all dependencies are explicitly listed in TanStack Query hook dependency arrays, similar to the React `useEffect` exhaustive deps rule. It helps prevent stale closures and ensures queries/mutations re-run when their dependencies change.

```javascript
// Invalid - missing `userId` in dependency array
const { data } = useQuery({
  queryKey: ['user'],
  queryFn: () => fetchUser(userId),
});

// Valid - all dependencies explicitly listed
const { data } = useQuery({
  queryKey: ['user', userId],
  queryFn: () => fetchUser(userId),
});
```