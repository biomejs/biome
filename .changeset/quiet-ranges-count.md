---
"@biomejs/biome": patch
---

Fixed `.editorconfig` section globs that have a negative number in a numeric range. `[{-3..3}]` now matches the files named `-3` to `3`, instead of only a file named `{-3..3}`.
