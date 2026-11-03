---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueTextareaMustache`](https://biomejs.dev/linter/rules/no-vue-textarea-mustache/), which disallows `{{ }}` text interpolation inside a Vue `<textarea>`. Vue doesn't support it there; use `v-model` instead.

```vue
<template>
  <textarea>{{ message }}</textarea>
</template>
```
