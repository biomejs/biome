---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedHtmlElementIs`](https://biomejs.dev/linter/rules/no-vue-deprecated-html-element-is/), which disallows the `is` attribute on HTML elements in Vue templates. Vue 3 no longer renders a component in place of these elements, unless the value starts with `vue:`.

```vue
<template>
  <tr is="my-row"></tr>
</template>
```
