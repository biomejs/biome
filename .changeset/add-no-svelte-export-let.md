---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteExportLet`](https://biomejs.dev/linter/rules/no-svelte-export-let/), which disallows declaring Svelte component props with the legacy `export let` syntax. Use the `$props()` rune instead.

```svelte
<script>
  export let name;
</script>
```
