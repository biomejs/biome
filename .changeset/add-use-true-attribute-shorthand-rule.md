---
"@biomejs/biome": patch
---

Added the new nursery rule [`useTrueAttributeShorthand`](https://biomejs.dev/linter/rules/use-true-attribute-shorthand/), which requires the shorthand form when a Vue component prop is set to `true`. The `style` option can require the longhand form instead.

```vue
<template>
  <MyComponent :disabled="true" />
</template>
```
