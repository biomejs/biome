---
"@biomejs/biome": patch
---

Fixed [#9653](https://github.com/biomejs/biome/issues/9653): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer report false positives on Svelte `{#snippet}` parameters, snippet names, or `{@render ...}` calls that reference a snippet defined in the same file.

The following snippet no longer triggers either rule:

```svelte
{#snippet figure(image)}
    <figure>
        <img src={image.src} alt={image.caption} />
        <figcaption>{image.caption}</figcaption>
    </figure>
{/snippet}

{#each images as img}
    {@render figure(img)}
{/each}
```
