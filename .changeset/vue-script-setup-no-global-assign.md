---
"@biomejs/biome": patch
---

`noGlobalAssign` no longer reports assignments to a Vue `<script setup>` binding from a template expression, when the binding's name happens to match a built-in global (e.g. `open`, `parent`, `top`).

For example, this no longer triggers a diagnostic:

```vue
<script setup>
const open = defineModel();
</script>

<template>
  <button @click="open = !open">Toggle</button>
</template>
```
