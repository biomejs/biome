---
"@biomejs/biome": patch
---

Fixed [#9370](https://github.com/biomejs/biome/issues/9370): The resolver now correctly prioritizes more specific `exports` patterns over less specific ones. Previously, a pattern like `"./*"` could match before `"./features/*"`, causing resolution failures for packages with overlapping subpath patterns.
