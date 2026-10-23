---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedPropsDefaultThis`](https://biomejs.dev/linter/rules/no-vue-deprecated-props-default-this/), which reports `this` inside the function that computes the default value of a Vue prop. Vue doesn't give this function access to the component, so `this` doesn't point to it. Read other props from the function's first argument instead.

```vue
<script>
export default {
  props: {
    size: Number,
    sizes: {
      type: Array,
      default() {
        return [this.size];
      },
    },
  },
};
</script>
```
