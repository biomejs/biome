---
"@biomejs/biome": patch
---

Fix `lint/suspicious/noArrayIndexKey` to report index usage anywhere in JSX `key` template or binary expressions, not only in the last visited identifier.
