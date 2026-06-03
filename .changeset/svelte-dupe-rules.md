---
"@biomejs/biome": patch
---

Added three new nursery rules for Svelte templates:

- [`noDupeElseIfBlocks`](https://biomejs.dev/linter/rules/no-dupe-else-if-blocks/): disallow duplicate conditions in `{#if}` / `{:else if}` chains. A condition that is textually identical to a previous one can never execute.
- [`noDupeStyleProperties`](https://biomejs.dev/linter/rules/no-dupe-style-properties/): disallow duplicate `style:` directives on the same element.
- [`noDupeUseDirectives`](https://biomejs.dev/linter/rules/no-dupe-use-directives/): disallow duplicate `use:` directives on the same element.
