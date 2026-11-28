---
"@biomejs/biome": patch
---

Fixed a false positive in [`useTanstackQueryExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-tanstack-query-exhaustive-dependencies/): the rule no longer reports the variable that holds the query key when the query function reads it. Previously, the fix added the variable to its own value.

```js
function useTodo(id, queryClient) {
    const queryKey = ["todo", id];
    return useQuery({
        queryKey,
        queryFn: () =>
            fetchTodo(id).then((todo) => {
                queryClient.setQueryData(queryKey, todo); // No longer reported.
                return todo;
            }),
    });
}
```
