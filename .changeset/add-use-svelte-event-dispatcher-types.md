---
"@biomejs/biome": patch
---

Added the nursery rule [`useSvelteEventDispatcherTypes`](https://biomejs.dev/linter/rules/use-svelte-event-dispatcher-types/), which reports `createEventDispatcher()` calls that don't declare the types of their events in Svelte components written in TypeScript.

The rule accepts a call that declares its events:

```svelte
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ save: string }>();
</script>
```
