---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueChildContent`](https://biomejs.dev/linter/rules/no-vue-child-content/), which disallows content inside elements that use `v-html` or `v-text`. These directives replace the content of their element, so Vue discards anything written there.

```vue
<template>
  <div v-html="html">Loading...</div>
</template>
```
