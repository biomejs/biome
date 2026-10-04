---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueShadowNativeEvents`](https://biomejs.dev/linter/rules/no-vue-shadow-native-events/), which reports Vue component events named after native DOM events, such as `click` or `keydown`.

```vue
<script setup>
// Invalid
const emit = defineEmits(["click"]);
emit("click");
</script>
```
