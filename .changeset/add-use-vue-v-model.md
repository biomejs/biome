---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueVModel`](https://biomejs.dev/linter/rules/use-vue-v-model/), which reports a prop binding on a Vue component paired with an `update:` listener that only writes the new value back, and suggests `v-model` instead.

```vue
<template>
  <MyComponent :title="title" @update:title="title = $event" />
</template>
```
