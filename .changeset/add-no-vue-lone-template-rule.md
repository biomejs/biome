---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueLoneTemplate`](https://biomejs.dev/linter/rules/no-vue-lone-template/), which reports `<template>` elements inside a Vue template that don't use `v-if`, `v-else-if`, `v-else`, `v-for`, or `v-slot`. Vue renders these as regular HTML `<template>` elements, so their content isn't shown.

```vue
<template>
  <template>
    <p>Hello</p>
  </template>
</template>
```
