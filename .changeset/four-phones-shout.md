---
"@biomejs/biome": patch
---

Fixed [#6492](https://github.com/biomejs/biome/issues/6492). The
`organizeImports` assist action no longer duplicates a comment at the start of
the file when `:BLANK_LINE:` precedes the first import group.
