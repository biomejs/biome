---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueVIfWithVFor`](https://biomejs.dev/linter/rules/no-vue-v-if-with-v-for/). This rule disallows `v-for` and `v-if` on the same element.

```vue
<!-- Invalid -->
<div v-for="item in items" v-if="item.isActive">
  {{ item.name }}
</div>
```
