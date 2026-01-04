---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueOptionsApi`](https://biomejs.dev/linter/rules/no-vue-options-api/).

The rule disallows Vue Options API properties (`data`, `methods`, `computed`, lifecycle hooks) which are incompatible with Vue 3.6's Vapor Mode. For example:

```vue
<!-- Invalid -->
<script>
export default {
  data() { return { count: 0 } }
}
</script>

<!-- Valid -->
<script setup>
import { ref } from 'vue'
const count = ref(0)
</script>
```
