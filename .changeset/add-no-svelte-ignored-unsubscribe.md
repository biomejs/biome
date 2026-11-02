---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteIgnoredUnsubscribe`](https://biomejs.dev/linter/rules/no-svelte-ignored-unsubscribe/), which reports calls to a store's `subscribe()` method when the returned function is ignored. Without that function, the subscription can never be stopped, which causes a memory leak.

```svelte
<script>
  import { count } from "./stores.js";

  count.subscribe((value) => console.log(value));
</script>
```
