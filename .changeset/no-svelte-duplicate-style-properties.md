---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteDuplicateStyleProperties`](https://biomejs.dev/linter/rules/no-svelte-duplicate-style-properties/) for Svelte templates: disallow duplicate `style:` directives on the same element. `<div style:color="red" style:color="blue"></div>` is invalid.
