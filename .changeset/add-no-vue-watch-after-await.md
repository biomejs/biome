---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueWatchAfterAwait`](https://biomejs.dev/linter/rules/no-vue-watch-after-await/), which disallows creating watchers after an `await` in a Vue component's `setup()` function. Vue doesn't stop these watchers when the component is removed. A watcher is allowed when the function it returns is stored, so that function can be called later to stop the watcher.

```vue
<script>
import { watch } from "vue";

export default {
  async setup() {
    await fetchUser();
    watch(source, () => {});
  },
};
</script>
```
