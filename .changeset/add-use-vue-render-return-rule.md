---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueRenderReturn`](https://biomejs.dev/linter/rules/use-vue-render-return/), which requires the `render` function of a Vue component to return a value on every path.

The following example is invalid because the `render` function returns nothing when `this.visible` is `false`:

```vue
<script>
import { h } from "vue";

export default {
  render() {
    if (this.visible) {
      return h("div", "hello");
    }
  },
};
</script>
```
