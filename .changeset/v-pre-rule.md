---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueValidVPre`](https://biomejs.dev/linter/rules/use-vue-valid-v-pre/). Enforces that usages of the `v-pre` directive in Vue.js SFC are valid.

```vue
<!-- Valid -->
<div v-pre />

<!-- Invalid -->
<div v-pre:aaa />
<div v-pre.bbb />
<div v-pre="ccc" />
```
