---
"@biomejs/biome": patch
---

Fixed [#8812](https://github.com/biomejs/biome/issues/8812): `lint/suspicious/noArrayIndexKey` will now report index usage anywhere in JSX `key` template or binary expressions, not only in the last visited identifier.
