---
"@biomejs/biome": patch
---

Fixed [#9901](https://github.com/biomejs/biome/issues/9901): the command `lint --write` is now idempotent when it's run against HTML-ish files that contains scripts and styles.
