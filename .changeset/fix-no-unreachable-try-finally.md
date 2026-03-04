---
"@biomejs/biome": patch
---

Fixed [#4946](https://github.com/biomejs/biome/issues/4946): `noUnreachable` no longer reports code inside `finally` blocks as unreachable when there is a `break`, `continue`, or `return` in the corresponding `try` body.
