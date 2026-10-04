---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteDomManipulating`](https://biomejs.dev/linter/rules/no-svelte-dom-manipulating/), which disallows direct DOM manipulation of elements bound with `bind:this` in Svelte components.

```svelte
<script>
  let element;
  const remove = () => element.remove();
</script>

<div bind:this={element}></div>
```
