---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueEmitValidator`](https://biomejs.dev/linter/rules/use-vue-emit-validator/), which requires a validator function for each event declared in the `emits` option or in `defineEmits()`.

```vue
<script setup>
// Reported: the event "change" has no validator function.
defineEmits(["change"]);
</script>
```
