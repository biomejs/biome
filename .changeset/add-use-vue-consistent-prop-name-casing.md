---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueConsistentPropNameCasing`](https://biomejs.dev/linter/rules/use-vue-consistent-prop-name-casing/), which requires prop names in Vue components to use one casing style. By default, prop names must be in camelCase. The `casing` option switches the rule to snake_case.

```vue
<script setup>
defineProps({
  greeting_text: String,
});
</script>
```
