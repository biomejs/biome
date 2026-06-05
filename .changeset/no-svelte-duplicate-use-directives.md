---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteDuplicateUseDirectives`](https://biomejs.dev/linter/rules/no-svelte-duplicate-use-directives/) for Svelte templates: disallow duplicate `use:` directives on the same element. `<div use:tooltip use:tooltip></div>` is invalid.
