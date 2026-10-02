---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteObjectInTextMustaches`](https://biomejs.dev/linter/rules/no-svelte-object-in-text-mustaches/), which disallows object, array, function, and class literals in Svelte text mustaches, where they are rendered as strings such as `[object Object]`.

```svelte
<!-- Invalid: renders "[object Object]" -->
{{ name }}
```
