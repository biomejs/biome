---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidVIs`](https://biomejs.dev/linter/rules/use-vue-valid-v-is/), which reports `v-is` directives that have an argument or modifiers, have no value, or are used on a component instead of a native HTML element.

For example, the following `v-is` directive has no value:

```vue
<template>
  <tr v-is></tr>
</template>
```
