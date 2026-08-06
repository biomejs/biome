---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteLegacyConst`](https://biomejs.dev/linter/rules/no-svelte-legacy-const/), which disallows legacy Svelte `{@const}` tags and recommends declaration tags with `$derived()`.

Invalid:

```svelte
{#each boxes as box}
  {@const area = box.width * box.height}
  <p>{area}</p>
{/each}
```

Valid:

```svelte
{#each boxes as box}
  {const area = $derived(box.width * box.height)}
  <p>{area}</p>
{/each}
```
