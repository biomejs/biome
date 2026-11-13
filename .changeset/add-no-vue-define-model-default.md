---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDefineModelDefault`](https://biomejs.dev/linter/rules/no-vue-define-model-default/), which disallows the `default` option in Vue's `defineModel()`. When a parent component passes `undefined` with `v-model`, the child component uses the default value while the parent keeps `undefined`, so the two components show different values.

```vue
<script setup>
const model = defineModel({ type: String, default: "" });
</script>
```
