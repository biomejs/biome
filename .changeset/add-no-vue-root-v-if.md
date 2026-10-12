---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRootVIf`](https://biomejs.dev/linter/rules/no-vue-root-v-if/), which disallows `v-if` on the only root element of a Vue component template.

```vue
<template>
  <div v-if="visible">Content</div>
</template>
```
