---
"@biomejs/biome": patch
---

Added the nursery rule `useArraySome` to prefer `.some()` over verbose existence checks like `filter(...).length > 0` and `findIndex(...) !== -1`, with suggestions for `find`/`findLast` existence checks. This also applies to ES2025 iterator helpers such as `Iterator.prototype.find`.
