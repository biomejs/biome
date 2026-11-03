---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueUndeclaredEmits`](https://biomejs.dev/linter/rules/no-vue-undeclared-emits/), which reports events that a Vue component emits without declaring them in the `emits` option or in `defineEmits()`.

```vue
<script setup>
const emit = defineEmits(["change"]);

emit("close"); // reported: "close" is not declared
</script>
```
