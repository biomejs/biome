---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueUselessVBind`](https://biomejs.dev/linter/rules/no-vue-useless-v-bind/), which reports `v-bind` directives whose value is a fixed string. A plain attribute does the same thing.

```vue
<template>
  <div :id="'foo'" /> <!-- reported, use id="foo" instead -->
</template>
```
