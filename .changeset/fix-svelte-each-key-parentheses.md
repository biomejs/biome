---
"@biomejs/biome": patch
---

Fixed parsing of closing parentheses in Svelte `{#each}` block key expressions. Biome now correctly parses method calls and other parenthesised expressions used as keys.

For example, the following snippets are now parsed correctly:

```svelte
{#each numbers as number, index (number.toString())}
  <p>{number}</p>
{/each}

{#each numbers as number (key(number))}
  <p>{number}</p>
{/each}
```
