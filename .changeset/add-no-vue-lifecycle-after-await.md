---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueLifecycleAfterAwait`](https://biomejs.dev/linter/rules/no-vue-lifecycle-after-await/), which reports Vue lifecycle hooks, such as `onMounted()`, that are registered after an `await` in a component's `setup()` function. Vue doesn't run these hooks.

```vue
<script>
import { onMounted } from "vue";

export default {
  async setup() {
    await fetchUser();
    onMounted(() => {});
  },
};
</script>
```
