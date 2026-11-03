---
"@biomejs/biome": patch
---

Fixed `.editorconfig` section globs that skipped the character following a bracket expression. For example, `[{[ab],c}.json]` now matches `a.json`, `b.json`, and `c.json`.
