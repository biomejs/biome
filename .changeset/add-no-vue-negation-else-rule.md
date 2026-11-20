---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueNegationElse`](https://biomejs.dev/linter/rules/no-vue-negation-else/), which reports a negated condition in `v-if` or `v-else-if` when a `v-else` branch follows it. Its fix removes the negation and swaps the two branches.

```vue
<template>
  <div v-if="!isLoading">Content</div> <!-- reported -->
  <div v-else>Loading...</div>
</template>
```
