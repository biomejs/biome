---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRefObjectReactivityLoss`](https://biomejs.dev/linter/rules/no-vue-ref-object-reactivity-loss/), which reports reads of a Vue ref's value that run only once, in the same function that creates the ref. Such a read copies the current value, and the copy doesn't update when the ref changes.

```js
import { ref } from "vue";

const count = ref(0);
const doubled = count.value * 2;
```
