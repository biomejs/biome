---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRequiredPropWithDefault`](https://biomejs.dev/linter/rules/no-vue-required-prop-with-default/), which reports Vue props that are marked as required but also have a default value.

```vue
<script setup lang="ts">
const props = withDefaults(defineProps<{ name: string }>(), {
  name: "World",
});
</script>
```
