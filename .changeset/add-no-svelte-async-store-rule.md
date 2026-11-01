---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteAsyncStore`](https://biomejs.dev/linter/rules/no-svelte-async-store/), which disallows passing async functions to the `writable()`, `readable()`, and `derived()` stores from `svelte/store`.

```js
import { writable } from "svelte/store";
const user = writable(null, async (set) => {
    set(await fetchUser());
});
```
