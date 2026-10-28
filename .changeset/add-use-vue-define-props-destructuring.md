---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueDefinePropsDestructuring`](https://biomejs.dev/linter/rules/use-vue-define-props-destructuring/), which requires destructuring the props returned by `defineProps()` in Vue `<script setup>` blocks. It also reports `withDefaults()`, because destructured props can set their default values directly.

```vue
<script setup lang="ts">
const props = defineProps<{ foo?: string }>();
</script>
```
