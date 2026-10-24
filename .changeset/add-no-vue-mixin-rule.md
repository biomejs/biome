---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueMixin`](https://biomejs.dev/linter/rules/no-vue-mixin/), which reports the `mixins` option in Vue components. In Vue 3, composables are the recommended way to share logic between components. The rule is recommended and part of the `vue` domain.

```vue
<script>
export default {
  mixins: [counterMixin],
};
</script>
```
