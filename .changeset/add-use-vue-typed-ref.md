---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueTypedRef`](https://biomejs.dev/linter/rules/use-vue-typed-ref/), which reports `ref()` and `shallowRef()` calls in TypeScript that have no type argument and no initial value other than `null` or `undefined`. Without a type, TypeScript can't tell which values these refs are meant to hold.

```ts
import { ref } from "vue";

const count = ref();
```
