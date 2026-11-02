---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedEventsApi`](https://biomejs.dev/linter/rules/no-vue-deprecated-events-api/), which reports calls to the `$on`, `$off`, and `$once` instance methods in Vue components. Vue 3 removed these methods, so calling them throws an error.

```vue
<script>
export default {
  mounted() {
    this.$on("start", handler); // reported
  },
};
</script>
```
