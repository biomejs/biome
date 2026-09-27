---
"@biomejs/biome": patch
---

Fixed [#11947](https://github.com/biomejs/biome/issues/11947): the `noDuplicateObjectKeys` and `noDuplicateClassMembers` rules now normalize numeric property names to their JavaScript property key before comparison. Previously, `{ 0x1: "a", 1: "b" }` and `{ 1.0: "a", 1: "b" }` were not reported as duplicates even though both keys become `"1"` at runtime.
