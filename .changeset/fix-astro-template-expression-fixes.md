---
"@biomejs/biome": patch
---

Fixed [#9181](https://github.com/biomejs/biome/issues/9181). When HTML full support is enabled, Biome now applies unsafe fixes to expressions embedded in `{...}` template expressions of `.astro` files. Previously the fix for a rule such as [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) was computed but never written back into the document.
