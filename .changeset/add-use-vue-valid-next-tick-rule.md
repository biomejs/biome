---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVueValidNextTick`](https://biomejs.dev/linter/rules/use-vue-valid-next-tick/), which reports Vue `nextTick` calls that don't wait for Vue to update the page. The rule is recommended and part of the Vue domain.

```js
import { nextTick } from "vue";

nextTick(); // reported: the Promise is ignored
nextTick.then(callback); // reported: `nextTick` isn't called
```
