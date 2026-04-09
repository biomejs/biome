---
"@biomejs/biome": patch
---

Fixed [#9136](https://github.com/biomejs/biome/issues/9136) and [#9653](https://github.com/biomejs/biome/issues/9653): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer report false positives on several Svelte template constructs that declare or reference bindings in the host grammar:

- `{#snippet name(params)}` — the snippet name and its parameters (including object, array, rest, and nested destructuring) are now tracked.
- `{@render name(args)}` — the snippet name used at the render site is now resolved against the snippet declaration.
- `{#each items as item, index (key)}` — the `item` binding (plain identifier or destructured), the optional `index`, and the optional `key` expression are now tracked.
- `{@const name = value}` — the declared name is now tracked as a binding and the initializer is analyzed for undeclared references.
- `{@debug a, b, c}` — each debugged identifier is now analyzed and reported if undeclared.
- Shorthand attributes `<img {src} />` — the curly-shorthand attribute is now analyzed as an expression, so undeclared references inside it are reported.

For example, the following template no longer triggers either rule:

```svelte
<script>
let items = [];
let total = 0;
</script>

{#snippet figure(image)}
    <figure>
        <img src={image.src} alt={image.caption} />
        <figcaption>{image.caption}</figcaption>
    </figure>
{/snippet}

{#each items as item}
    {@const price = item.price}
    {@render figure(item)}
    <span>{price}</span>
{/each}

{@debug items, total}
```
