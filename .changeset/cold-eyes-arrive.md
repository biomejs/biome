---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueBooleanDefault`](https://biomejs.dev/linter/rules/no-vue-boolean-default/), which disallows default values for Boolean props in Vue components. Vue already defaults an absent Boolean prop to `false`.

```vue
<script setup lang="ts">
withDefaults(defineProps<{ disabled?: boolean }>(), {
  disabled: true,
});
</script>
```
