---
"@biomejs/biome": patch
---

Improved the parsing of Svelte templates. Biome now parses the binding of an `{#each}` block as JavaScript, so it supports default values, computed keys, nested rest elements, and TypeScript type annotations, for example `{#each items as { [key]: [value = fallback, ...rest] }: Item, index (value)}`. Text expressions that start with a regular expression or a comment, such as `{/^a/.test(name)}`, are also supported.
