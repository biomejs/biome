---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueConsistentVForDelimiter`](https://biomejs.dev/linter/rules/use-vue-consistent-v-for-delimiter/), which requires the same word, `in` or `of`, between the item and the list in every Vue `v-for` directive. The default is `in`, and the `delimiter` option switches it to `of`.

```vue
<!-- reported by default -->
<div v-for="item of items"></div>
```
