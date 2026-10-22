---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDuplicateVElseIf`](https://biomejs.dev/linter/rules/no-vue-duplicate-v-else-if/), which reports `v-else-if` conditions in Vue templates that repeat, or are covered by, earlier conditions of the same `v-if` / `v-else-if` chain.

For example, the second element below can never be rendered:

```vue
<template>
  <div v-if="a || b" />
  <div v-else-if="a" />
</template>
```
