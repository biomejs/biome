---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueConsistentVSlotStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-slot-style/), which enforces one style for Vue slots: `#name`, `v-slot:name`, or `v-slot` for the default slot. By default, it expects `v-slot` on a component and `#` on `<template>` elements.

```vue
<template>
  <MyComponent>
    <template v-slot:header>Title</template>
  </MyComponent>
</template>
```
