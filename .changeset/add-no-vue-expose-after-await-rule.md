---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueExposeAfterAwait`](https://biomejs.dev/linter/rules/no-vue-expose-after-await/), which reports `expose()` and `defineExpose()` calls that come after an `await` in a Vue component's setup code. These calls run too late, so a parent component might not receive the exposed properties.

```vue
<script setup>
await fetchUser();
defineExpose({ reload });
</script>
```
