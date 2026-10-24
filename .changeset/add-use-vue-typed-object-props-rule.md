---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueTypedObjectProps`](https://biomejs.dev/linter/rules/use-vue-typed-object-props/), which reports Vue props declared with `Object` or `Array` without a TypeScript type in TypeScript files and Vue files with `<script lang="ts">`.

```vue
<script setup lang="ts">
defineProps({
  user: Object, // reported
  items: Array as PropType<string[]>,
});
</script>
```
