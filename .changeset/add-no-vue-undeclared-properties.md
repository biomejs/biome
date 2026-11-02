---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueUndeclaredProperties`](https://biomejs.dev/linter/rules/no-vue-undeclared-properties/), which reports names that a Vue component uses but doesn't declare, in its template, in `this.name`, and in `props.name`.

```vue
<script setup>
const count = 0;
</script>

<template>
  <!-- `cnt` isn't declared -->
  <p>{{ cnt }}</p>
</template>
```
