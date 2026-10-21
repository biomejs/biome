---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRestrictedCallAfterAwait`](https://biomejs.dev/linter/rules/no-vue-restricted-call-after-await/), which reports calls to configured functions that come after an `await` in a Vue component's `setup()` function.

For example, with `{ "functions": [{ "module": "vue-i18n", "path": "useI18n" }] }`, the following call is reported:

```vue
<script>
import { useI18n } from "vue-i18n";

export default {
  async setup() {
    await fetchUser();
    const { t } = useI18n();
    return { t };
  },
};
</script>
```
