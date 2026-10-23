---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedDeleteSet`](https://biomejs.dev/linter/rules/no-vue-deprecated-delete-set/), which reports calls to the `$set` and `$delete` functions, `Vue.set` and `Vue.delete`, and `set` and `del` imported from `vue`. Vue 3 removed these functions, so calling them throws an error.

```vue
<script>
export default {
  mounted() {
    this.$set(this.user, "name", "Alice");
  },
};
</script>
```
