---
"@biomejs/biome": patch
---

Added the nursery rule [`useSvelteKitResolve`](https://biomejs.dev/linter/rules/use-svelte-kit-resolve/), which requires internal links and calls to `goto()`, `pushState()`, and `replaceState()` to use paths built with SvelteKit's `resolve()`.

```svelte
<a href="/about">About</a>
```
