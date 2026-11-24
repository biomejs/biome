---
"@biomejs/biome": patch
---

Fixed [#13384](https://github.com/biomejs/biome/issues/13384): stabilized the opt-in Svelte support enabled by `html.experimentalFullSupportEnabled`. Biome now parses, formats, lints, applies fixes, and respects suppression comments in `.svelte`, `.svelte.js`, and `.svelte.ts` files consistently across CLI and editor integrations, with compatibility tested against Svelte 5.59.1. Formatting is limited to whole files; range and on-type formatting are not yet supported.
