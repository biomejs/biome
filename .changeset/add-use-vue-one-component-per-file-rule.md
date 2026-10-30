---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueOneComponentPerFile`](https://biomejs.dev/linter/rules/use-vue-one-component-per-file/), which reports files that define more than one Vue component. The rule counts `<script setup>` blocks, `export default { ... }` in `.vue` files, and `defineComponent()` and `createApp()` calls that define a component in place with an options object or a setup function. It doesn't report the main component of a `.vue` file.

```vue
<script setup>
import { defineComponent, h } from "vue";

const Badge = defineComponent({
  render: () => h("span", "New"),
});
</script>
```
