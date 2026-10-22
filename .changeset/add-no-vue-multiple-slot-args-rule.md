---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueMultipleSlotArgs`](https://biomejs.dev/linter/rules/no-vue-multiple-slot-args/), which disallows passing more than one argument to a Vue slot from a render function. Slot content written in a template receives only the first argument.

```vue
<script>
export default {
  render() {
    return this.$slots.default(item, index); // reported
  },
};
</script>
```
