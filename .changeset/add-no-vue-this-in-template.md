---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueThisInTemplate`](https://biomejs.dev/linter/rules/no-vue-this-in-template/), which reports `this` in Vue templates. Templates can read the properties of the component directly, so `this.text` can be written as `text`:

```vue
<template>
  <p>{{ this.text }}</p>
</template>
```
