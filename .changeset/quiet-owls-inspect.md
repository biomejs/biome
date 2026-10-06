---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteInspect`](https://biomejs.dev/linter/rules/no-svelte-inspect/), which reports uses of the Svelte `$inspect` rune, such as `$inspect(count)` or `$inspect.trace()`, left over from debugging.
