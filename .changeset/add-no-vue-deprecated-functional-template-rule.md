---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedFunctionalTemplate`](https://biomejs.dev/linter/rules/no-vue-deprecated-functional-template/), which reports the `functional` attribute on the `<template>` of a Vue component. Vue 3 no longer supports this attribute.

```vue
<template functional>
  <div>{{ props.message }}</div>
</template>
```
