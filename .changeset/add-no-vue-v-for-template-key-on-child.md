---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueVForTemplateKeyOnChild`](https://biomejs.dev/linter/rules/no-vue-v-for-template-key-on-child/), which reports a `key` placed on a child element of a `<template v-for>` instead of on the `<template>` tag.

```vue
<template>
  <template v-for="todo in todos">
    <TodoItem :key="todo.id" />
  </template>
</template>
```
