---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueValidVOnce`](https://biomejs.dev/linter/rules/use-vue-valid-v-once/). Enforces that usages of the `v-once` directive in Vue.js SFC are valid.

```vue
<!-- Valid -->
<div v-once />

<!-- Invalid -->
<div v-once:aaa />
<div v-once.bbb />
<div v-once="ccc" />
```
