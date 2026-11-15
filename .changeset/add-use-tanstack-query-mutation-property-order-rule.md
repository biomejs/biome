---
"@biomejs/biome": patch
---

Added the nursery rule [`useTanstackQueryMutationPropertyOrder`](https://biomejs.dev/linter/rules/use-tanstack-query-mutation-property-order/), which requires `onMutate` to come before `onError` and `onSettled` in the options of TanStack Query's `useMutation`. The type of the value that `onMutate` returns can be wrong in a property that comes before it.

```js
import { useMutation } from "@tanstack/react-query";

useMutation({
  mutationFn: (todo) => saveTodo(todo),
  onError: (error, todo, saved) => restoreTodo(saved.previousTodo),
  onMutate: (todo) => ({ previousTodo: readTodo(todo.id) }),
});
```
