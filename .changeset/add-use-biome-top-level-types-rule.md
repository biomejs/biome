---
"@biomejs/biome": patch
---

Added the nursery rule [`useBiomeTopLevelTypes`](https://biomejs.dev/linter/rules/use-biome-top-level-types/), which reports the use of `nodeKinds()`, `tokenKinds()`, or `union()` inside a function. The rule belongs to the `biomePlugin` domain.

```js
import { defineRule, nodeKinds } from "@biomejs/runtime/plugin";

export const rule = defineRule({
  run(node) {
    return nodeKinds("JS_CALL_EXPRESSION").has(node);
  },
});
```
