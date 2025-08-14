---
"@biomejs/biome": patch
---

Fixed a bug where when using project rules, the file scanner incorrectly indexed ignored files, when those
files were part of other ignored folders/files.
