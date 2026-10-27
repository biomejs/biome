---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueTypedProps`](https://biomejs.dev/linter/rules/use-vue-typed-props/), which reports Vue props declared without a type, in the `props` option, `defineProps()`, or `defineModel()`. Without a type, Vue accepts any value for the prop and can't warn when a component receives the wrong kind of value.

```vue
<script setup>
defineProps(["title"]);
</script>
```
