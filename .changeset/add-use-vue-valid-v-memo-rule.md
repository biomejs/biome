---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidVMemo`](https://biomejs.dev/linter/rules/use-vue-valid-v-memo/), which reports invalid `v-memo` directives in Vue templates. A `v-memo` directive must have an array value, can't have an argument or modifiers, and doesn't work inside an element with `v-for`.

For example, the following value is an object instead of an array:

```vue
<template>
  <div v-memo="{ id }"></div>
</template>
```
