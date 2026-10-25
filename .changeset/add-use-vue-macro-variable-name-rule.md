---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueMacroVariableName`](https://biomejs.dev/linter/rules/use-vue-macro-variable-name/), which requires specific variable names for the results of `defineProps()`, `defineEmits()`, `defineSlots()`, `useSlots()` and `useAttrs()` in Vue `<script setup>` blocks. The names can be changed with the rule's options.

```vue
<script setup>
const propsDefined = defineProps({ msg: String }); // reported, should be `props`
</script>
```
