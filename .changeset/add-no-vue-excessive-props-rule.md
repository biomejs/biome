---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueExcessiveProps`](https://biomejs.dev/linter/rules/no-vue-excessive-props/), which reports Vue components that declare more props than the `maxProps` option allows. The default limit is `10`.

```vue
<!-- With "maxProps": 2 -->
<script setup lang="ts">
defineProps<{ title: string; subtitle: string; icon: string }>(); // reported
</script>
```
