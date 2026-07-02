---
"@biomejs/biome": patch
---

Improved type-aware lint rule inference for built-in globals and indexed function calls. Biome now resolves `Error(...)`, `new Error(...)`, optional `Error#stack`, and calls through indexed function values such as `handlers[0]()` more accurately.
