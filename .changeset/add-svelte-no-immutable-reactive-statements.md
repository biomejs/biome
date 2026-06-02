---
"@biomejs/biome": patch
---

Added the new nursery rule [`noSvelteImmutableReactiveStatements`](https://biomejs.dev/linter/rules/no-svelte-immutable-reactive-statements/), which reports Svelte reactive statements (`$:`) that reference only immutable values, since they never re-run.

```svelte
<script>
const base = 1;
$: doubled = base * 2;
</script>
```
