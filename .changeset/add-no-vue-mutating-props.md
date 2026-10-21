---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueMutatingProps`](https://biomejs.dev/linter/rules/no-vue-mutating-props/), which reports code that changes the props of a Vue component, in the script or in the template. Props belong to the parent component, so the child should emit an event or copy the value instead.

```vue
<script setup>
const props = defineProps({ items: Array });

props.items.push("new item");
</script>
```
