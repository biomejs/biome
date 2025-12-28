---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueValidVCloak`](https://biomejs.dev/linter/rules/use-vue-valid-v-cloak/). Enforces that usages of the `v-cloak` directive in Vue.js SFC are valid.

```vue
<!-- Valid -->
<div v-cloak />

<!-- Invalid -->
<div v-cloak:aaa />
<div v-cloak.bbb />
<div v-cloak="ccc" />
```
