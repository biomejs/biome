---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteAddEventListener`](https://biomejs.dev/linter/rules/no-svelte-add-event-listener/), which reports `addEventListener` calls in Svelte files and suggests the `on` function from `svelte/events` instead.

```svelte
<script>
  window.addEventListener("resize", handler);
</script>
```
