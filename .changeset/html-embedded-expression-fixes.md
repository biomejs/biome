---
"@biomejs/biome": patch
---

Fixed [#9181](https://github.com/biomejs/biome/issues/9181): fixes from JavaScript rules are now applied inside `{…}` and `{{…}}` template expressions of Astro, Svelte, and Vue files. Previously the diagnostics were reported but `--write` left the expressions unchanged.
