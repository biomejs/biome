---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueDeprecatedVIs`](https://biomejs.dev/linter/rules/no-vue-deprecated-v-is/), which reports the deprecated Vue `v-is` directive. Use the `is` attribute with a `vue:` prefix instead.

```vue
<template>
  <div v-is="'MyComponent'"></div>
</template>
```
