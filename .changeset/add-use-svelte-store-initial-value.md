---
"@biomejs/biome": patch
---

Added the nursery rule [`useSvelteStoreInitialValue`](https://biomejs.dev/linter/rules/use-svelte-store-initial-value/), which requires `writable()`, `readable()`, and `derived()` stores from `svelte/store` to be created with an initial value.

```js
import { writable } from "svelte/store";

export const count = writable();
```
