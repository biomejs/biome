---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueValidVShow`](https://biomejs.dev/linter/rules/use-vue-valid-v-show/), which reports `v-show` directives that have an argument, a modifier, or no value, or that are placed on a `<template>` element.

```vue
<template v-show="isVisible"><div></div></template>
```
