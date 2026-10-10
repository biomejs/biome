---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteClassDirective`](https://biomejs.dev/linter/rules/no-svelte-class-directive/), which reports Svelte's `class:` directive. Since Svelte 5.16, the `class` attribute accepts objects and arrays, and Svelte recommends them over the directive.

```svelte
<div class:active={isActive}></div>
```
