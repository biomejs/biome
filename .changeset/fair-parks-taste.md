---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueRefAsOperand`](https://biomejs.dev/linter/rules/no-vue-ref-as-operand/). This rule disallows cases where a ref is used as an operand.

The following code is now flagged:

```js
import { ref } from "vue"

const count = ref(0);
count++; // Should be: count.value++
```

```js
import { ref } from "vue"

const ok = ref(false);
if (ok) { // Should be: if (ok.value)
  //
}
```
