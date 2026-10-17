---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueComponentIs`](https://biomejs.dev/linter/rules/use-vue-component-is/), which requires Vue `<component>` elements to bind the `is` attribute with `:is` or `v-bind:is`. Without it, Vue has no component to render.

```vue
<template>
  <component :is="currentView" />
</template>
```
