---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueComputedPropertyReturn`](https://biomejs.dev/linter/rules/use-vue-computed-property-return/), which requires the getters of Vue computed properties to return a value on every path.

```vue
<script setup>
import { computed, ref } from "vue";

const count = ref(0);
const double = computed(() => {
  count.value * 2; // reported: the getter never returns a value
});
</script>
```
