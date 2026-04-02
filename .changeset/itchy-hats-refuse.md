---
"@biomejs/biome": patch
---

Fixed [#9781](https://github.com/biomejs/biome/issues/9781): Trailing comments after a top-level `biome-ignore-all format` suppression are now preserved instead of being dropped. This applies to JavaScript, CSS, HTML, JSONC, GraphQL, and Grit files.
