---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidDefineEmits`](https://biomejs.dev/linter/rules/use-vue-valid-define-emits/), which enforces valid usage of Vue's `defineEmits` macro in `<script setup>`.

For example, the following code is now reported, because `defineEmits` declares its events with both a type and an argument:

```vue
<script setup lang="ts">
const emit = defineEmits<(e: "notify") => void>(["submit"]);
</script>
```
