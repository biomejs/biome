---
"@biomejs/biome": patch
---

Fixed [#13089](https://github.com/biomejs/biome/issues/13089): `useValidAnchor` in HTML, Astro, Svelte, and Vue no longer reports anchor elements that use spread attributes or dynamic bindings, matching JSX behavior.
