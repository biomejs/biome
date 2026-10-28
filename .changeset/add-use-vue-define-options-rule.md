---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueDefineOptions`](https://biomejs.dev/linter/rules/use-vue-define-options/), which reports the default export of a plain `<script>` block in Vue components that also use `<script setup>`, and suggests setting those options with `defineOptions()` instead.

```vue
<script>
export default { name: "MyComponent" }; // reported
</script>

<script setup>
const props = defineProps(["title"]);
</script>
```
