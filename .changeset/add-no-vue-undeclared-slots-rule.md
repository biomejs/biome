---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueUndeclaredSlots`](https://biomejs.dev/linter/rules/no-vue-undeclared-slots/), which reports `<slot>` elements in Vue components written in TypeScript when the slot isn't declared with `defineSlots()` or with the `slots` option.

```vue
<script setup lang="ts">
defineSlots<{
  default(props: { msg: string }): any;
}>();
</script>

<template>
  <slot name="footer" /> <!-- reported -->
</template>
```
