---
"@biomejs/biome": patch
---

Fixed [#11436](https://github.com/biomejs/biome/issues/11436): GritQL snippets such as `export { $specifiers } from $source` now match named re-exports with aliases, inline `type` modifiers, and multiple specifiers.
