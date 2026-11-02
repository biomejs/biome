---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedInlineTemplate`](https://biomejs.dev/linter/rules/no-vue-deprecated-inline-template/), which disallows the `inline-template` attribute on Vue components. Vue 3 removed this feature, so the content between the component's tags is no longer used as its template.

```vue
<template>
  <my-component inline-template>
    <div>{{ message }}</div>
  </my-component>
</template>
```
