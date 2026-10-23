---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueEmitsValidatorReturn`](https://biomejs.dev/linter/rules/use-vue-emits-validator-return/), which reports Vue event validators that never return a true value. Vue prints a warning in development mode each time such an event is emitted.

```vue
<script setup>
defineEmits({
  submit(payload) {
    payload.email.includes("@");
  },
});
</script>
```
