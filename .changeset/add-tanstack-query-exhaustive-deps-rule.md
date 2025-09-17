---
"@biomejs/biome": minor
---

Added the rule [`useTanStackQueryExhaustiveDeps`](https://biomejs.dev/linter/rules/use-tanstack-query-exhaustive-deps/).

This rule ensures that all dependencies used by the `queryFn` are explicitly represented in the `queryKey`, similar to React’s `useEffect` exhaustive-deps rule. It helps prevent stale closures and ensures queries re-run when their dependencies change.

```javascript
// Invalid - missing `userId` in dependency array
const { data } = useQuery({
  queryKey: ["user"],
  queryFn: () => fetchUser(userId),
});

// Valid - all dependencies explicitly listed
const { data } = useQuery({
  queryKey: ["user", userId],
  queryFn: () => fetchUser(userId),
});
```
