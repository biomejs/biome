---
"@biomejs/biome": patch
---

Fixed [#11947](https://github.com/biomejs/biome/issues/11947): [`noDuplicateObjectKeys`](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) and [`noDuplicateClassMembers`](https://biomejs.dev/linter/rules/no-duplicate-class-members/) now report duplicate keys and class members when numeric or BigInt literals use different forms for the same property name.
