---
"@biomejs/biome": patch
---

Added the nursery rule [`useSvelteReactiveClasses`](https://biomejs.dev/linter/rules/use-svelte-reactive-classes/), which reports `Date`, `Map`, `Set`, `URL`, and `URLSearchParams` objects that are modified in Svelte files or exported from Svelte modules. Svelte doesn't detect changes made inside these built-in objects, so the rule suggests the matching classes from `svelte/reactivity`.

```svelte
<script>
  const tags = new Set();
  function addTag(tag) {
    tags.add(tag);
  }
</script>
```
