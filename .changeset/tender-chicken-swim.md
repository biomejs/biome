---
"@biomejs/biome": patch
---

Fixed [#9910](https://github.com/biomejs/biome/issues/9910): added support for parsing member expressions in Svelte directive properties. Biome now correctly parses directives like `in:renderer.in|global`, `use:obj.action`, and deeply nested forms like `in:a.b.c|global`.
