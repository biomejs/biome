---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedFilter`](https://biomejs.dev/linter/rules/no-vue-deprecated-filter/), which reports the filter syntax that Vue 3 removed.

For example, the following template uses the `currency` filter:

```vue
<template>
  <p>{{ price | currency }}</p>
</template>
```
