---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueTemplateKey`](https://biomejs.dev/linter/rules/no-vue-template-key/), which disallows the `key` attribute on `<template>` elements in Vue files. A `key` on a `<template>` that also uses `v-for` is still allowed.

```vue
<template>
  <template :key="item.id"><div /></template>
</template>
```
