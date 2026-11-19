---
"@biomejs/biome": patch
---

Fixed [#13303](https://github.com/biomejs/biome/issues/13303): `formatContent()` from the JS API now matches `overrides` for file paths with Windows separators, such as `src\a.js`. Backslashes in a file path are treated as path separators when matching configuration globs.
