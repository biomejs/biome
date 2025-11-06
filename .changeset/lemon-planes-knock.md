---
"@biomejs/biome": patch
---

Refactored formatter to use strict `Token` element for better performance. The new `Token` variant is optimized for static, ASCII-only text (keywords, operators, punctuation) with the following constraints:

- ASCII only (no Unicode characters)
- No newlines (`\n`, `\r`)
- No tab characters (`\t`)

This enables faster printing and fitting logic by using bulk string operations (`push_str`, `len()`) instead of character-by-character iteration with Unicode width calculations.
