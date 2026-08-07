---
"@biomejs/biome": patch
---

Fixed [#11275](https://github.com/biomejs/biome/issues/11275): Fixed a bug that could cause formatting Astro, Vue, and Svelte files in editors to delete markup when full HTML support is enabled in a nested configuration. Changes to embedded snippet settings now also take effect for already-open files.
