---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueConsistentDefineEmitsDeclaration`](https://biomejs.dev/linter/rules/use-vue-consistent-define-emits-declaration/), which enforces one declaration style for `defineEmits` in `<script setup lang="ts">`. By default, events must be declared with a type. The `style` option can instead require each event to be a property of that type, such as `change: [id: number]`, or require an argument, such as `defineEmits(["change"])`.

```vue
<script setup lang="ts">
const emit = defineEmits(["change", "update"]);
</script>
```
