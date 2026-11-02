---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueTemplateShadow`](https://biomejs.dev/linter/rules/no-vue-template-shadow/), which disallows `v-for` and `v-slot` variables that hide a variable with the same name from an outer scope of a Vue template.

```vue
<template>
  <div v-for="item in groups">
    <span v-for="item in item.children">{{ item }}</span>
  </div>
</template>
```
