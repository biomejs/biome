---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueDeprecatedScopeAttribute`](https://biomejs.dev/linter/rules/no-vue-deprecated-scope-attribute/), which reports the `scope` attribute on `<template>` elements in Vue files. Vue no longer supports this attribute, so use `v-slot` to receive slot props instead.

```vue
<template>
  <ListComponent>
    <template scope="props">{{ props.title }}</template>
  </ListComponent>
</template>
```
