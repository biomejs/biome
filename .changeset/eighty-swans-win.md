---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueConsistentDefinePropsDeclaration`](https://biomejs.dev/linter/rules/use-vue-consistent-define-props-declaration/), which enforces consistent `defineProps` declaration style.

### Invalid

```vue,expect_diagnostic
<script setup lang="ts">
const props = defineProps({
  kind: { type: String },
});
</script>
```

### Valid

```vue
<script setup lang="ts">
const props = defineProps<{
  kind: string;
}>();
</script>
