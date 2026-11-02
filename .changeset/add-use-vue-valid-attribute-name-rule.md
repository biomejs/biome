---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidAttributeName`](https://biomejs.dev/linter/rules/use-vue-valid-attribute-name/), which reports invalid attribute names in Vue templates, including names bound with `v-bind`. Attributes on components aren't checked.

```vue
<template>
  <p 0abc :-def="value"></p>
</template>
```
