---
"@biomejs/biome": patch
---

Fixed [#10045](https://github.com/biomejs/biome/issues/10045): CSS formatter no longer over-indents nested functional pseudo-classes such as `:not(:where(...))` when wrapping selector lists.
