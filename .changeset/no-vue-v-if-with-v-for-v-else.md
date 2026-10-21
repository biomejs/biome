---
"@biomejs/biome": patch
---

The rule [`noVueVIfWithVFor`](https://biomejs.dev/linter/rules/no-vue-v-if-with-v-for/) now also reports `v-else-if` and `v-else` on the same element as `v-for`.

```vue
<div v-if="loading">Loading...</div>
<TodoItem v-else v-for="todo in todos" :todo="todo" />
```
