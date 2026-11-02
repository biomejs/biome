---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedSlotAttribute`](https://biomejs.dev/linter/rules/no-vue-deprecated-slot-attribute/), which reports the `slot` attribute in Vue templates. Vue 3 no longer uses this attribute to fill named slots, so use `v-slot` (or `#`) on a `<template>` element instead.

```vue
<template>
  <MyComponent>
    <template slot="header">Title</template>
  </MyComponent>
</template>
```
