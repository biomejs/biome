---
"@biomejs/biome": patch
---

Added new rule [`useVueDefineMacrosOrder`](https://biomejs.dev/linter/rules/use-vue-define-macros-order) which allows enforcing specific order for Vue compiler macros.

In this example, the rule will suggest moving `defineProps` before `defineEmits`:
```vue
<script lang="ts" setup>
const emit = defineEmits(['update'])
const props = defineProps<{ name: string }>()
</script>
```
