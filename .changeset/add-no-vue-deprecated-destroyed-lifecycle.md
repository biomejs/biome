---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedDestroyedLifecycle`](https://biomejs.dev/linter/rules/no-vue-deprecated-destroyed-lifecycle/), which reports the `beforeDestroy` and `destroyed` lifecycle hooks in Vue components. Vue 3 renamed them to `beforeUnmount` and `unmounted`, and no longer calls hooks with the old names.

```vue
<script>
export default {
  destroyed() {},
};
</script>
```
