---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueUnusedEmitDeclarations`](https://biomejs.dev/linter/rules/no-vue-unused-emit-declarations/), which reports events that a Vue component declares but never emits.

For example, the following component never emits `close`:

```vue
<script setup>
const emit = defineEmits(["change", "close"]);
emit("change");
</script>
```
