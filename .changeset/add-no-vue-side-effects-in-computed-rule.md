---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueSideEffectsInComputed`](https://biomejs.dev/linter/rules/no-vue-side-effects-in-computed/), which reports Vue computed properties that change state. Vue decides when to run a computed property, so changes made inside one happen at unpredictable times.

```vue
<script setup>
import { computed, ref } from "vue";

const items = ref([]);
// `reverse()` changes `items.value` in place.
const reversedItems = computed(() => items.value.reverse());
</script>
```
