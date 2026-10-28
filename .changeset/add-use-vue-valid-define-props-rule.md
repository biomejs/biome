---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidDefineProps`](https://biomejs.dev/linter/rules/use-vue-valid-define-props/), which reports `defineProps()` calls that Vue can't compile or that declare props incorrectly. For example, it reports a component that calls `defineProps()` twice, or whose `defineProps()` argument uses a variable declared in `<script setup>`:

```vue
<script setup>
const definition = { msg: String };
defineProps(definition);
</script>
```
