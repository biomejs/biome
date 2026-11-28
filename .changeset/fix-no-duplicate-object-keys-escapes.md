---
"@biomejs/biome": patch
---

Fixed [#13487](https://github.com/biomejs/biome/issues/13487), where [`noDuplicateObjectKeys`](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) did not recognize duplicate object keys that use different escape representations (such as `\u0061` vs `a`, or UTF-16 surrogate pairs).
