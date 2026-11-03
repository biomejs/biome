---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedModelDefinition`](https://biomejs.dev/linter/rules/no-vue-deprecated-model-definition/), which reports the `model` option in Vue components. Vue 3 ignores this option, so `v-model` on the component always uses the `modelValue` prop and the `update:modelValue` event.

```vue
<script>
export default {
  model: {
    prop: "checked",
    event: "change",
  },
};
</script>
```
