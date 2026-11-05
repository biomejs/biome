---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRestrictedComponentOptions`](https://biomejs.dev/linter/rules/no-vue-restricted-component-options/), which reports Vue component options that are listed in its `componentOptions` option.

For example, with `"componentOptions": ["init", ["props", "*", "twoWay"]]`, the following component is reported twice:

```vue
<script>
export default {
  init() {},
  props: {
    name: { type: String, twoWay: true },
  },
};
</script>
```
