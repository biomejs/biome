---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueUnusedRefs`](https://biomejs.dev/linter/rules/no-vue-unused-refs/), which reports template refs in Vue components that nothing reads through `$refs`, `useTemplateRef()`, or a variable of the same name in `<script setup>`. The rule needs `html.experimentalFullSupportEnabled` to be `true`.

```vue
<template>
  <input ref="input" />
</template>
```
