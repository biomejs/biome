---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueVTextVHtmlOnComponent`](https://biomejs.dev/linter/rules/no-vue-v-text-v-html-on-component/), which disallows `v-text` and `v-html` on Vue components. These directives replace everything inside the component's root element instead of filling its slot.

```vue
<template>
  <MyComponent v-html="html"></MyComponent>
</template>
```
