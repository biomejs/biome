---
"@biomejs/biome": patch
---

Added a new nursery rule [`useVueNextTickPromise`](https://biomejs.dev/linter/rules/use-vue-next-tick-promise/), which enforces Promise syntax when using Vue `nextTick`.

For example, the following snippet triggers the rule:

```js
import { nextTick } from "vue";

nextTick(() => {
    updateDom();
});
```
