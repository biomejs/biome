---
"@biomejs/biome": patch
---

Fixed #8209: Recognized formatting capability when either range or on-type formatting is supported, not only full-file formatting. This ensures editors and the language server correctly detect formatting support in files like JSONC.
