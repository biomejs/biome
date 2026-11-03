---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueUselessTemplateAttributes`](https://biomejs.dev/linter/rules/no-vue-useless-template-attributes/), which reports attributes and directives that have no effect on a `<template>` element with `v-if`, `v-else-if`, `v-else`, `v-for`, or `v-slot` in Vue files. Vue renders only the content of such an element, so it ignores the other attributes.

```vue
<template>
  <template v-if="visible" class="heading">Title</template>
</template>
```
