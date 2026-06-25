---
"@biomejs/biome": patch
---

Added the new nursery rule [`noSvelteUnnecessaryStateWrap`](https://biomejs.dev/linter/rules/no-svelte-unnecessary-state-wrap/), which reports unnecessary `$state()` wrapping of classes from `svelte/reactivity` that are already reactive.

```svelte
<script>
import { SvelteMap } from "svelte/reactivity";
const map = $state(new SvelteMap()); // redundant
</script>
```
