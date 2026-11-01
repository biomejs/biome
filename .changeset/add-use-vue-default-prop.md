---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueDefaultProp`](https://biomejs.dev/linter/rules/use-vue-default-prop/), which reports optional props of Vue components that have no default value. Props that only accept `Boolean` are not reported.

For example, the following component doesn't say what `size` is when a parent component leaves it out:

```vue
<script setup>
defineProps({
  size: Number,
});
</script>
```
