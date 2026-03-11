---
"@biomejs/biome": patch
---

Fixed [#9432](https://github.com/biomejs/biome/issues/9432): Values referenced as a JSX element in Astro/Vue/Svelte templates are now correctly detected; `noUnusedImports` and `useImportType` rules no longer reports these values as false positives.
