---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueConsistentBooleanPropStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-boolean-prop-style/), which enforces a consistent style for Vue component props set to `true`. By default, it requires the shorthand form; the `style` option can require the longhand form instead.

```vue
<template>
  <MyComponent :disabled="true" />
</template>
```
