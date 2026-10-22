---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueTemplateRef`](https://biomejs.dev/linter/rules/use-vue-template-ref/), which requires `useTemplateRef()` instead of `ref()` or `shallowRef()` to access elements of the template in `<script setup>`.

```vue
<template>
  <input ref="input" />
</template>

<script setup>
import { ref } from "vue";

const input = ref(); // reported
</script>
```
