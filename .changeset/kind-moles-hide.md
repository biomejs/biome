---
"@biomejs/biome": patch
---

Fixed [#10082](https://github.com/biomejs/biome/issues/10082): [`noAssignInExpressions`](https://biomejs.dev/linter/rules/no-assign-in-expressions/) no longer flags assignments inside Svelte `{@const name = value}` blocks. Those assignments are declarations scoped to the enclosing block, not accidental side effects.

The following pattern is now considered valid:

```svelte
{#each items as item}
  {@const doubled = item * 2}
  <div>{doubled}</div>
{/each}
```
