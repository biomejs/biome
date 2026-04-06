---
"@biomejs/biome": patch
---

Fixed [#8685](https://github.com/biomejs/biome/issues/8685) where `noUselessLoneBlockStatements` would remove empty blocks containing comments. The rule now preserves these blocks since comments may contain important information like TODOs or commented-out code.
