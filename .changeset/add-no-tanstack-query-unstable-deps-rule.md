---
"@biomejs/biome": patch
---

Added the new nursery rule [`noTanstackQueryUnstableDeps`](https://biomejs.dev/linter/rules/no-tanstack-query-unstable-deps/), which reports the object returned by a TanStack Query hook, such as `useQuery` or `useMutation`, when it is used directly as a dependency of `useEffect`, `useMemo`, or `useCallback`. This object can be a new object on every render, so React treats the dependencies as changed on every render.

```js
const mutation = useMutation({ mutationFn });
const callback = useCallback(() => mutation.mutate("hello"), [mutation]);
```
