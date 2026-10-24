---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueMultipleObjectsInClass`](https://biomejs.dev/linter/rules/no-vue-multiple-objects-in-class/), which reports `:class` arrays in Vue templates that contain more than one object. Merge the conditional classes into a single object instead.

```vue
<template>
  <div :class="[{ active: isActive }, { disabled: isDisabled }]" />
</template>
```
