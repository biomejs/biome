---
"@biomejs/biome": patch
---

Fixed [`noVueWatchAfterAwait`](https://biomejs.dev/linter/rules/no-vue-watch-after-await/), [`noVueLifecycleAfterAwait`](https://biomejs.dev/linter/rules/no-vue-lifecycle-after-await/) and [`noVueRestrictedCallAfterAwait`](https://biomejs.dev/linter/rules/no-vue-restricted-call-after-await/), which now also check the `setup()` function passed as the first argument of `defineComponent()`. The following code is now reported:

```js
import { defineComponent, onMounted } from "vue";

defineComponent(async (props) => {
  await load();
  onMounted(() => {});
}, { props: ["id"] });
```

When `defineComponent()` receives both a function and an options object, these rules now ignore the `setup` option, because Vue runs the function instead.
