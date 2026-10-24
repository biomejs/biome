---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueBlockOrder`](https://biomejs.dev/linter/rules/use-vue-block-order/), which enforces the order of the top-level blocks in Vue components. By default, `<script>` and `<template>` blocks come first, in any order, followed by `<style>` blocks. The order can be changed with the `order` option.

```vue
<style></style>
<script></script>
```
