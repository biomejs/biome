---
"@biomejs/biome": patch
---

Fixed [`noVueUndeclaredDirectives`](https://biomejs.dev/linter/rules/no-vue-undeclared-directives/) not reporting undeclared custom directives in components that export their options with `export { options as default }`.

```vue
<script>
const options = {};
export { options as default };
</script>

<template>
  <div v-highlight></div>
</template>
```
