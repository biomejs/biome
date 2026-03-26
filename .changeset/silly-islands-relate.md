---
"@biomejs/biome": patch
---

Fixed duplicate parse errors in `check` and `ci` output. When a file had syntax errors, the same parse error was printed twice and the error count was inflated.
