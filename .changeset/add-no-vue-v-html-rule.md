---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueVHtml`](https://biomejs.dev/linter/rules/no-vue-v-html/), which disallows Vue's `v-html` directive. `v-html` inserts its value into the page as HTML, so text from users can run code on your page (cross-site scripting, or XSS).

```vue
<template>
  <div v-html="content"></div>
</template>
```
