---
"@biomejs/biome": patch
---

Fixed [#13089](https://github.com/biomejs/biome/issues/13089): The [`useValidAnchor`](https://biomejs.dev/linter/rules/use-valid-anchor/) rule no longer reports anchor elements in Astro, Svelte, or Vue templates when they use a spread attribute or an object `v-bind` directive.
