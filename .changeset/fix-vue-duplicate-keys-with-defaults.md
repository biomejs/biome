---
"@biomejs/biome": patch
---

Fixed a false positive in [`noVueDuplicateKeys`](https://biomejs.dev/linter/rules/no-vue-duplicate-keys/) where variables created from props wrapped in `withDefaults()` were reported as duplicates of those props. The following code is now valid:

```vue
<script setup lang="ts">
import { toRefs } from "vue";

const props = withDefaults(defineProps<{ text: string }>(), {});
const { text } = toRefs(props);
</script>
```
