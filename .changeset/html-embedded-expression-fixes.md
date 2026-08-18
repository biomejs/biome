---
"@biomejs/biome": patch
---

Fixed [#9181](https://github.com/biomejs/biome/issues/9181): fixes from JavaScript rules such as [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) and [`useTemplate`](https://biomejs.dev/linter/rules/use-template/) are now applied inside `{…}` and `{{…}}` template expressions of Astro, Svelte, and Vue files. Previously the diagnostics were reported, but `--write` left the expressions unchanged.
