---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueConsistentVOnHandlerStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-on-handler-style/), which enforces one style of event handler in Vue `v-on` directives. By default, it prefers the name of a method, and an inline function where a method name can't be used.

```vue
<template>
  <button @click="save()">Save</button>
</template>
```
