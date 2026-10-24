---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidVModel`](https://biomejs.dev/linter/rules/use-vue-valid-v-model/), which reports `v-model` directives that Vue can't use: on unsupported elements or file inputs, with an argument or unknown modifier on a native element, without a value, or with a value that can't be assigned to.

For example, the following `v-model` directives are invalid:

```vue
<template>
  <div v-model="foo"></div>
  <input v-model="foo + bar" />
</template>
```
